pub mod data;
extern crate serde;

use data::Data as Datas;
use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;
use matrix_sdk::{
    Client, config::SyncSettings, room::Room,
    ruma::RoomId,
    ruma::RoomAliasId,
    ruma::events::room::{
        message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent},
    },
    self,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("amx [option] [x]")
        .command(
            Command::new("timeline")
            .usage("amx timeline")
            .description("timeline (ex : $amx t)")
            .action(t)
            .alias("t")
            .flag(
                Flag::new("bot", FlagType::Bool)
                .description("bot flag (ex: $ amx t -bot)")
                .alias("bot"),
                )
            )
        .command(
            Command::new("post")
            .usage("amx post {}")
            .description("post room (ex: $ amx p message -j '#example:matrix.org')")
            .action(p)
            .alias("p")
            .flag(
                Flag::new("join", FlagType::String)
                .description("join flag (ex: $ amx p message -j '#example:matrix.org')")
                .alias("j"),
                )
            .flag(
                Flag::new("id", FlagType::String)
                .description("join room_id flag (ex: $ amx p message -i '!example:matrix.org')")
                .alias("i"),
                )
            )
            .command(
            Command::new("room")
            .usage("amx room")
            .description("join room (ex: $ amx r -j '#example:matrix.org')")
            .action(r)
            .alias("r")
            .flag(
                Flag::new("join", FlagType::String)
                .description("join flag (ex: $ amx r -j '#example:matrix.org')")
                .alias("j"),
                )
            .flag(
                Flag::new("id", FlagType::String)
                .description("join room_id flag (ex: $ amx r message -i '!example:matrix.org')")
                .alias("i"),
                )
            )
        ;
    app.run(args);
}

async fn amx_timeline(event: OriginalSyncRoomMessageEvent, room: Room) {
    let Room::Joined(room) = room else { return };
    let room_id = room.room_id();
    let u = event.sender;
    let c_body = event.content;
    let body = c_body.body();
    let name = room.name();
    let alias = room.canonical_alias();
    let content = room.tombstone();
    let room_avatar = room.avatar_url();
    println!("{:#?}", c_body);
    println!("{:#?} {} {} {}",alias, room_id, u, body);
    println!("{:#?}", room_avatar);
    println!("{:#?}", name);
    println!("{:#?}", content);
    println!("{:#?}", room_avatar);
}

// "!party"とpostされると"test"と返す
async fn amx_bot(event: OriginalSyncRoomMessageEvent, room: Room) {
    let Room::Joined(room) = room else { return };
    let MessageType::Text(text_content) = event.content.msgtype else { return };
    if text_content.body.contains("!party") {
        let content = RoomMessageEventContent::text_plain("test");
        println!("sending");
        room.send(content, None).await.unwrap();
        println!("message sent");
    }
}

#[allow(unused_must_use)]
async fn amx_timeline_client(homeserver_url: String, username: &str, password: &str, c: &Context) -> anyhow::Result<()> {
    #[allow(unused_mut)]
    let mut client_builder = Client::builder().homeserver_url(homeserver_url);

    #[cfg(feature = "sled")]
    {
        let home = dirs::home_dir().expect("no home directory found").join("amx");
        client_builder = client_builder.sled_store(home, None)?;
    }

    #[cfg(feature = "indexeddb")]
    {
        client_builder = client_builder.indexeddb_store("amx", None).await?;
    }

    let client = client_builder.build().await?;
    client
        .login_username(username, password)
        .initial_device_display_name("amx")
        .send()
        .await?;
    let sync_token = client.sync_once(SyncSettings::default()).await.unwrap().next_batch;

    if c.bool_flag("bot") {
        client.add_event_handler(amx_bot);
    } else {
        client.add_event_handler(amx_timeline);
    }

    let settings = SyncSettings::default().token(sync_token);
    client.sync(settings).await?;
    client.sync(SyncSettings::default()).await?;
    Ok(())
}

#[tokio::main]
async fn amx_timeline_login(c: &Context) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let data = Datas::new().unwrap();
    let homeserver_url = (&data.home_server).to_string();
    amx_timeline_client(homeserver_url, &data.username, &data.password, c).await?;
    Ok(())
}

fn t(c: &Context){
    let client = amx_timeline_login(c).unwrap();
    println!("{:#?}", client);
}

#[allow(unused_must_use)]
async fn amx_post_client(homeserver_url: String, username: &str, password: &str, c: &Context) -> anyhow::Result<()> {
    #[allow(unused_mut)]
    let mut client_builder = Client::builder().homeserver_url(homeserver_url);
    #[cfg(feature = "sled")]
    {
        let home = dirs::home_dir().expect("no home directory found").join("amx");
        client_builder = client_builder.sled_store(home, None)?;
    }
    #[cfg(feature = "indexeddb")]
    {
        client_builder = client_builder.indexeddb_store("amx", None).await?;
    }

    let client = client_builder.build().await?;
    client
        .login_username(username, password)
        .initial_device_display_name("amx")
        .send()
        .await?;
    client.sync_once(SyncSettings::default()).await.unwrap().next_batch;
    let message = c.args[0].to_string();
    let content = RoomMessageEventContent::text_plain(&message);
    if let Ok(join) = c.string_flag("join") {
        let join: &str = &join;
        let room_alias = <&RoomAliasId>::try_from(join).unwrap();
        let room = client.resolve_room_alias(&room_alias).await?;
        let room_id = room.room_id;
        println!("{:#?}", room_id);
        if let Some(room) = client.get_joined_room(&room_id) {
            println!("{:#?}", join);
            room.send(content, None).await?;
        }
    } else if let Ok(id) = c.string_flag("id") {
        let id: &str = &id;
        let room_id = <&RoomId>::try_from(id).unwrap();
        println!("{:#?}", room_id);
        if let Some(room) = client.get_joined_room(&room_id) {
            room.send(content, None).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn amx_post_login(c: &Context) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let data = Datas::new().unwrap();
    let homeserver_url = (&data.home_server).to_string();
    amx_post_client(homeserver_url, &data.username, &data.password, c).await?;
    Ok(())
}

fn p(c: &Context) {
    let client = amx_post_login(c).unwrap();
    println!("{:#?}", client);
}

async fn amx_room_client(homeserver_url: String, username: &str, password: &str, c: &Context) -> anyhow::Result<()> {

    #[allow(unused_mut)]
    let mut client_builder = Client::builder().homeserver_url(homeserver_url);
    #[cfg(feature = "sled")]
    {
        let home = dirs::home_dir().expect("no home directory found").join("amx");
        client_builder = client_builder.sled_store(home, None)?;
    }
    #[cfg(feature = "indexeddb")]
    {
        client_builder = client_builder.indexeddb_store("amx", None).await?;
    }
    let client = client_builder.build().await?;
    client
        .login_username(username, password)
        .initial_device_display_name("amx")
        .send()
        .await?;
    client.sync_once(SyncSettings::default()).await.unwrap().next_batch;

    if let Ok(join) = c.string_flag("join") {
        let join: &str = &join;
        let room_alias = <&RoomAliasId>::try_from(join).unwrap();
        let room = client.resolve_room_alias(&room_alias).await?;
        let room_id = room.room_id;
        println!("{:#?}", room_id);
        client.join_room_by_id(&room_id).await?;
    } else if let Ok(id) = c.string_flag("id") {
        let id: &str = &id;
        let room_id = <&RoomId>::try_from(id).unwrap();
        client.join_room_by_id(&room_id).await?;
    } else {
        let a = client.joined_rooms();
        println!("{:#?}", a);
    }
    Ok(())
}

#[tokio::main]
async fn amx_room_login(c: &Context) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let data = Datas::new().unwrap();
    let homeserver_url = (&data.home_server).to_string();
    amx_room_client(homeserver_url, &data.username, &data.password, c).await?;
    Ok(())
}

fn r(c: &Context) {
    let client = amx_room_login(c).unwrap();
    println!("{:#?}", client);
}
