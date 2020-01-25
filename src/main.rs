use std::env::args;

use futures::future::{ok, Future};
use futures::stream;
use futures::stream::Stream;
use glitch_in_the_matrix::errors::MatrixError;
use glitch_in_the_matrix::request::MatrixRequestable;
use glitch_in_the_matrix::room::{NewRoom, RoomClient};
use glitch_in_the_matrix::sync::SyncStream;
use glitch_in_the_matrix::MatrixClient;
use gm_types::content::Content;
use gm_types::messages::Message;
use gm_types::replies::SendReply;
use gm_types::room::Room;
use gm_types::sync::{JoinedRoom, SyncReply};
use serde::{Deserialize, Serialize};
use serde_yaml;
use tokio_core::reactor::Core;
use urlencoding::encode;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    token: String,
    password: String,
    room: String,
    account: String,
    listen_to: String,
    prepend_with: String,
}

fn into_sends<T: MatrixRequestable + 'static>(
    jroom: &JoinedRoom,
    room_client: &mut RoomClient<T>,
    listen_to: String,
    prepend_with: String,
) -> Vec<impl Future<Item = SendReply, Error = MatrixError> + 'static> {
    jroom
        .timeline
        .events
        .iter()
        .filter_map(move |event| match &event.content {
            Content::RoomMessage(message) => match message {
                Message::Text {
                    body,
                    ..
                } => {
                    if body.starts_with(&listen_to) {
                        let crate_name = &body[listen_to.len()..];

                        println!("{}", crate_name);

                        Some(
                            room_client
                                .send_simple(format!("{}{}", prepend_with, crate_name))
                                .map_err(|e| {
                                    println!("send_err: {}", e);
                                    e
                                }),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn send_stream(
    (mut client, room): (MatrixClient, Room<'static>),
    listen_to: String,
    prepend_with: String,
) -> Box<dyn Stream<Item = impl Future<Item = SendReply, Error = MatrixError>, Error = MatrixError>>
{
    Box::new(
        SyncStream::new(client.clone())
            .map(move |freply: SyncReply| {
                let mut rc = RoomClient {
                    room: &room,
                    cli: &mut client,
                };

                let futs = if let Some(jroom) = freply.rooms.join.get(&room) {
                    into_sends(jroom, &mut rc, listen_to.clone(), prepend_with.clone())
                } else {
                    vec![]
                };

                stream::iter_ok(futs.into_iter())
            })
            .flatten(),
    )
}

#[allow(unused_mut)]
fn main() -> Result<(), std::io::Error> {
    let mut core = Core::new()?;

    let args: Config = serde_yaml::from_reader(std::fs::File::open(
        args().nth(1).unwrap_or_else(|| "config.yaml".into()),
    )?)
    .expect("Config file was not deserialisable.");
    let args2 = args.clone();

    let handle = core.handle();
    let handle2 = core.handle();
    let rm = args.room.clone();

    let txns =
        MatrixClient::new_from_access_token(&args.token, "https://matrix.org", &handle)
            .or_else(move |mut _e| {
                let handle2 = handle2.clone();

                MatrixClient::login_password(
                    &args.account,
                    &args.password,
                    "https://matrix.org",
                    &handle2,
                )
            })
            .and_then(move |mut client| {
                println!("Access token: {}", client.get_access_token());

                NewRoom::from_alias(&mut client, &encode(&rm)).map(move |room| (client, room))
            })
            .into_stream()
            .map(move |pair| {
                let args2 = args2.clone();

                send_stream(pair, args2.listen_to, args2.prepend_with)
            })
            .map_err(|e| println!("send_stream err: {:?}", e));

    let handle = core.handle();

    let res = txns.for_each(move |mut syncs| {
        let handle = handle.clone();

        syncs
            .for_each(move |txn| {
                handle.spawn(txn.map(|_| ()).map_err(|e| println!("txn err: {:?}", e)));

                ok(())
            })
            .or_else(|e| {
                println!("syncs error: {:?}", e);

                ok(())
            })
    });

    core.run(res).expect("Unresolved errors encountered.");

    Ok(())
}
