use std::env::args;

use futures::future::{ok, Future};
use futures::stream;
use futures::stream::Stream;
use gm_types::content::Content;
use gm_types::messages::Message;
use gm_types::replies::SendReply;
use gm_types::room::Room;
use gm_types::sync::{JoinedRoom, SyncReply};
use glitch_in_the_matrix::errors::MatrixError;
use glitch_in_the_matrix::request::MatrixRequestable;
use glitch_in_the_matrix::room::{NewRoom, RoomClient};
use glitch_in_the_matrix::sync::SyncStream;
use glitch_in_the_matrix::MatrixClient;
use serde::{Serialize, Deserialize};
use serde_yaml;
use tokio_core::reactor::Core;
use urlencoding::encode;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    token: String,
    password: String,
    room: String,
    account: String,
}


fn into_sends<T: MatrixRequestable + 'static>(
    jroom: &JoinedRoom,
    room_client: &mut RoomClient<T>
) -> Vec<impl Future<Item=SendReply, Error=()> + 'static> {
    jroom.timeline.events.iter().map(move |event| {
        match &event.content {
            Content::RoomMessage(message) => {
                match message {
                    Message::Text { body, formatted_body: _, format: _ } => {
                        if body.starts_with("!crate ") {
                            let crate_name = &body[7..];

                            println!("{}", crate_name);

                            Some(room_client.send_simple(
                                format!("https://crates.io/crates/{}", crate_name)
                            ).map_err(|e| { println!("{}", e); () }))
                        } else {
                            None
                        }
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }).filter_map(|f| f).collect()
}

fn send_stream(
    (mut client, room): (MatrixClient, Room<'static>)
) -> Box<dyn Stream<Item=impl Future<Item=SendReply, Error=()>, Error=MatrixError>> {
    Box::new(SyncStream::new(client.clone()).map(move |freply: SyncReply| {
        let mut rc = RoomClient { room: &room, cli: &mut client };

        let futs = if let Some(jroom) = freply.rooms.join.get(&room) {
            into_sends(jroom, &mut rc)
        } else {
            vec![]
        };
        
        stream::iter_ok(futs.into_iter())
    }).flatten())
}


#[allow(unused_mut)]
fn main() -> Result<(), std::io::Error> {
    let mut core = Core::new()?;

    let args: Config = serde_yaml::from_reader(
        std::fs::File::open(args().nth(1).unwrap_or("config.yaml".into()))?
    ).expect("Config file was not deserialisable.");

    let handle = core.handle();
    let handle2 = core.handle();
    let rm = args.room.clone();

    let txns = MatrixClient::new_from_access_token(
        &args.token,
        "https://matrix.org",
        &handle.clone()
    ).or_else(move |mut _e| {
        let handle2 = handle2.clone();
        let args = args.clone();

        MatrixClient::login_password(
            &args.account,
            &args.password,
            "https://matrix.org",
            &handle2
        )
    }).and_then(move |mut client| {
        let rm = rm.clone();
        
        println!("Access token: {}", client.get_access_token());

        NewRoom::from_alias(
            &mut client,
            &encode(&rm)
        ).map(move |room| (client, room))
    })
    .into_stream()
    .map(send_stream)
    .map_err(|_| ());

    let handle = core.handle();
 
    let res = txns.for_each(move |mut syncs| {
        let handle = handle.clone();

        syncs.map_err(|_e| { () }).for_each(move |txn| {
            handle.spawn(txn.map(|_| ()));

            ok(())
        })
    });

    core.run(res).expect("Failed to run txns");

    Ok(())
}
