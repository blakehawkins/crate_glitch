use std::env::args;
use std::io::Error;

use futures::future::Future;
use gm_types::messages::Message;
use glitch_in_the_matrix::request::MatrixRequestable;
use glitch_in_the_matrix::room::{NewRoom, RoomClient};
use glitch_in_the_matrix::MatrixClient;
use tokio_core::reactor::Core;
use urlencoding::encode;


fn main() -> Result<(), std::io::Error> {
    let mut core = Core::new()?;

    let txns = MatrixClient::login_password(
        "crates.io".into(),
        &args()
            .nth(1)
            .expect("Missing password argument"),
        "https://matrix.org",
        &core.handle()
    ).and_then(|mut client| {
        NewRoom::from_alias(&mut client, &encode("#_hack:matrix.org")).map(|room| (client, room))
    }).and_then(|(mut client, room)| {
        RoomClient {
            room: &room,
            cli: &mut client
        }.send(Message::Text { body: "hello world".into(), formatted_body: None, format: None })
    });

    core.run(txns).map_err(|merr| {
        println!("{}", merr);

        Error::last_os_error()
    }).expect("Failed to run txns");
    
    Ok(())
}
