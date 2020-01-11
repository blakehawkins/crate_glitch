use std::env::args;
use std::io::Error;

use futures::future::{ok, Future};
use futures::stream::Stream;
use gm_types::messages::Message;
use glitch_in_the_matrix::request::MatrixRequestable;
use glitch_in_the_matrix::room::{NewRoom, RoomClient};
use glitch_in_the_matrix::sync::SyncStream;
use glitch_in_the_matrix::MatrixClient;
use serde::{Serialize, Deserialize};
use serde_yaml;
use tokio_core::reactor::Core;
use urlencoding::encode;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    token: String,
    password: String,
}


fn main() -> Result<(), std::io::Error> {
    let mut core = Core::new()?;

    let args: Config = serde_yaml::from_reader(
        std::fs::File::open(args().nth(1).unwrap_or("config.yaml".into()))?
    ).expect("Config file was not deserialisable.");

    let handle = core.handle();

    let txns = MatrixClient::new_from_access_token(
        &args.token,
        "https://matrix.org",
        &handle
    ).or_else(|mut _e| MatrixClient::login_password(
        "crates.io".into(),
        &args.password,
        "https://matrix.org",
        &handle
    )).and_then(|mut client| {
        println!("Access token: {}", client.get_access_token());

        NewRoom::from_alias(
            &mut client,
            &encode("#_hack:matrix.org")
        ).map(|room| (client, room))
    }).and_then(|(mut client, room)| {
        let rc = RoomClient {
            room: &room,
            cli: &mut client
        };

        SyncStream::new(client).for_each(|freply| {
            println!("{:?}", freply);

            ok(())
        })
    });
        
//       .get_messages(
//           "IqwOCw7qM9_Rg3Fu9GHYKKK2kH38NI4YiOuysKc",
//           None,
//           false,
//           None
//       )
//       .send(
//           Message::Text {
//               body: "hello world".into(),
//               formatted_body: None,
//               format: None
//           }
//       )
//   }).for_each(|mreply| if let Some(reply) = mreply {
//       println!("{:?}", mreply);
//   });

    core.run(txns).map_err(|merr| {
        println!("{}", merr);

        Error::last_os_error()
    }).expect("Failed to run txns");
    
    Ok(())
}
