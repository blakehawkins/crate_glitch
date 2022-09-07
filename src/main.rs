use std::env::args;

use matrix_sdk::{
    config::SyncSettings,
    event_handler::Ctx,
    room::{Joined, Room},
    ruma::{
        events::room::message::{
            MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent,
            TextMessageEventContent,
        },
        UserId,
    },
    Client,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    token: String,
    password: String,
    room: String,
    account: String,
    listen_to: String,
    prepend_with: String,
}

async fn handle_listening_query(room: Joined, arg: String, args: &Config) -> Result<()> {
    room.send(RoomMessageEventContent::text_plain(format!("{}{}", args.prepend_with, arg)), None).await.unwrap();

    Ok(())
}

fn parse(input: &str, args: &Config) -> Result<String> {
    let mut input = input.split(' ');
    let command = input.next().context("Nothing was parsed")?;
    let arg = input.next().context("Second word was absent")?;

    if command == args.listen_to {
      return Ok(arg.into());
    }

    None.context("Not a command worth listening")
}

async fn on_room_message(
    event: OriginalSyncRoomMessageEvent,
    room: Room,
    args: Ctx<Config>,
) -> Result<()> {
    if let Room::Joined(room) = room {
        let msg_body = match event.content.msgtype {
            MessageType::Text(TextMessageEventContent { body, .. }) => body,
            _ => return Ok(()),
        };

        if let Ok(arg) = parse(&msg_body, &args) {
            handle_listening_query(room, arg, &args).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Config = serde_yaml::from_reader(std::fs::File::open(
        args().nth(1).unwrap_or_else(|| "config.yaml".into()),
    )?)
    .expect("Config file was not deserialisable.");
    
    let account_name = args.clone().account;
    let user = UserId::parse(account_name).context("invalid userid")?;
    let client = Client::builder().user_id(&user).build().await.context("Failed to build client")?;
    let password = args.clone().password;

    client.login(user, &password, None, None).await.context("Failed to login to homeserver")?;

    // Don't respond to old messages.
    client.sync_once(SyncSettings::default()).await.unwrap();

    client.register_event_handler_context(args).register_event_handler(on_room_message).await;

    client.sync(SyncSettings::default().token(client.sync_token().await.unwrap())).await;

    Ok(())
}
