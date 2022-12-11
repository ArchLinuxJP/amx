pub mod data;
extern crate serde;

use data::Data as Datas;
use data::Datam as Datasm;
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
use std::path::Path;
use std::fs;
use std::io::prelude::*;
use std::io;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    user: String,
    auto: bool,
    room: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("amx [option] [x]")
        .command(
            Command::new("accont")
            .usage("msr a {}")
            .description("account change (ex : $ amx a -d (<-account.toml), $ amx -s (<-social.toml), $ amx -a ~/test.toml")
            .alias("a")
            .action(a),
            )
        .command(
            Command::new("timeline")
            .usage("amx timeline")
            .description("timeline(ex : $amx t)")
            .action(t)
            .alias("t")
            .flag(
                Flag::new("type", FlagType::String)
                .description("type flag(ex: $ amx t --type {test,vimrc})")
                )
            .flag(
                Flag::new("join", FlagType::String)
                .description("join flag(ex: $ amx t -j #example:matrix.org)")
                .alias("j"),
                )
            .flag(
                Flag::new("user", FlagType::String)
                .description("user flag(ex: $ amx t --type vimrc -u @syui:syui.cf)")
                .alias("u"),
                )
            )
        .command(
            Command::new("post")
            .usage("amx post {}")
            .description("post room(ex: $ amx p message -j '#example:matrix.org')")
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
            .flag(
                Flag::new("room", FlagType::String)
                .description("room save flag(ex: $ amx p message -r #example:matrix.org)")
                .alias("r"),
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

fn get_domain_zsh() {
    let data = Datas::new().unwrap();
    let homeserver_url = (&data.home_server).to_string();
    let username = (&data.username).to_string();
    let e = "export MATRIX_BASE=".to_owned() + &homeserver_url + "\n";
    let e = e.to_string();
    let u = "export MATRIX_USER=".to_owned() + &username + "\n";
    let u = u.to_string();
    let f = shellexpand::tilde("~") + "/.config/amx/amx.zsh";
    let f = f.to_string();
    let r = shellexpand::tilde("~") + "/.config/amx/amx.zsh";
    let r = r.to_string();
    let fu = shellexpand::tilde("~") + "/.config/amx/amx.zsh";
    let fu = fu.to_string();
    fs::remove_file(r).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let mut f = fs::File::create(f).unwrap();
    f.write_all(e.as_bytes()).unwrap();
    let mut fu = fs::File::create(fu).unwrap();
    fu.write_all(u.as_bytes()).unwrap();
}

#[allow(unused_must_use)]
fn a(c: &Context)  {
    let i = c.args[0].to_string();
    let o = shellexpand::tilde("~") + "/.config/amx/config.toml";
    let o = o.to_string();
    if &i == "-d" {
        let i = shellexpand::tilde("~") + "/.config/amx/account.toml";
        let i = i.to_string();
        println!("{:#?} -> {:#?}", i, o);
        fs::copy(i, o);
    } else if &i == "-s" {
        let i = shellexpand::tilde("~") + "/.config/amx/social.toml";
        let i = i.to_string();
        println!("{:#?} -> {:#?}", i, o);
        fs::copy(i, o);
    } else {
        println!("{:#?} -> {:#?}", i, o);
        fs::copy(i, o);
    }
    get_domain_zsh();
}

fn amx_setting_user(c: &Context) -> io::Result<()> {
    let path = "/.config/amx/";
    let file = path.to_string() + &"setting.toml";
    let mut f = shellexpand::tilde("~").to_string();
    f.push_str(&file);
    println!("{}", f);
    let check = Path::new(&f).exists();
    if check == true {
        let o = fs::read_to_string(&f)?;
        println!("read {}", o);           
    }
    let mut f = fs::File::create(f)?;

    if let Ok(user) = c.string_flag("user") {
        let user = &*user.to_string();
        let setting = Setting {
            user: user.into(),
            auto: false,
            room: "".to_string(),
        };
        let toml = toml::to_string(&setting).unwrap();
        write!(f, "{}", toml)?;
        f.flush()?;
        println!("\n#TOML:\n{}", toml);
    } else {
        let setting = Setting {
            user: "all".to_string(),
            auto: true,
            room: "".to_string(),
        };
        let toml = toml::to_string(&setting).unwrap();
        write!(f, "{}", toml)?;
        f.flush()?;
        println!("\n#TOML:\n{}", toml);
    }
    Ok(())
}

#[allow(unused_must_use)]
fn amx_setting_room(c: &Context) -> io::Result<()> {
    let path = "/.config/amx/";
    let file = path.to_string() + &"setting.toml";
    let mut f = shellexpand::tilde("~").to_string();
    f.push_str(&file);
    println!("{}", f);
    let check = Path::new(&f).exists();
    if check == true {
        let o = fs::read_to_string(&f)?;
        println!("read {}", o);           
    }
    let data = Datasm::new().unwrap();
    let user = data.user;
    let auto = data.auto;
    let mut f = fs::File::create(f)?;

    if let Ok(room) = c.string_flag("room") {
        let room = &*room.to_string();
        let setting = Setting {
            user: user,
            auto: auto,
            room: room.into(),
        };
        let toml = toml::to_string(&setting).unwrap();
        write!(f, "{}", toml)?;
        f.flush()?;
        println!("\n#TOML:\n{}", toml);
    }
    Ok(())
}

async fn amx_timeline(event: OriginalSyncRoomMessageEvent, room: Room) {
    let Room::Joined(room) = room else { return };
    let room_id = room.room_id();
    let u = event.sender;
    let body = event.content.body();
    println!("{} {} {}", room_id, u, body);
}

// test-bot:!party
async fn amx_timeline_bot_test(event: OriginalSyncRoomMessageEvent, room: Room) {
    let Room::Joined(room) = room else { return };
    let MessageType::Text(text_content) = event.content.msgtype else { return };
    if text_content.body.contains("!party") {
        let content = RoomMessageEventContent::text_plain("test");
        println!("sending");
        room.send(content, None).await.unwrap();
        println!("message sent");
    }
}

fn vimrc_file_read(f: String, line: String, end: String){
    use std::process::Command;
    println!("{:#?}", end);
    println!("{:#?}", line);
    let r = shellexpand::tilde("~") + "/.config/amx/log.txt";
    let r = r.to_string();
    let check = Path::new(&r).exists();
    if check == true {
        fs::remove_file(r).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
    let awk = "NR==".to_owned() + &line + &",NR==" + &end;
    let output = Command::new("awk").arg(awk).arg(f).output().expect("awk");
    let o = String::from_utf8_lossy(&output.stdout);

    let o =  o.to_string();
    println!("{}", o);

    let l = shellexpand::tilde("~") + "/.config/amx/vimrc/log.txt";
    let l = l.to_string();
    let mut l = fs::File::create(l).unwrap();
    l.write_all(&o.as_bytes()).unwrap();
    println!("{:#?}", l);
}

fn vimrc_file_rm() {
    let path = "/.config/amx/vimrc/";
    let mut p = shellexpand::tilde("~").to_string();
    p.push_str(&path);
    let check = Path::new(&p).is_dir();
    if check == true {
        fs::remove_dir_all(path).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
}

fn vimrc_file_dl() {
    use std::process::Command;
    let path = "/.config/amx/vimrc/";
    let mut p = shellexpand::tilde("~").to_string();
    let mut s = shellexpand::tilde("~").to_string();
    let script = "/.config/amx/vimrc.zsh";
    p.push_str(&path);
    s.push_str(&script);
    println!("{:#?}", s);
    let check = Path::new(&p).is_dir();
    if check == false {
        Command::new("zsh").arg(s).spawn().expect("zsh");
    }
}

// vimrc-bot:!filename.vim#1-2
async fn amx_timeline_bot_vimrc(event: OriginalSyncRoomMessageEvent, room: Room) {
    let Room::Joined(room) = room else { return };
    let MessageType::Text(text_content) = event.content.msgtype else { return };

    let data = Datasm::new().unwrap();
    let s = data.user;
    let u = &event.sender;
    let auto = data.auto;

    if text_content.body.contains("!") {
        //↑  && || err panic
        if s == *u || auto == true {
            let t = text_content.body.split_inclusive('!').collect::<Vec<_>>();

            let file = &t[1].to_string();
            let tt = file.split_inclusive('#').collect::<Vec<_>>();
            let file = &tt[0];
            let line = &tt[1];
            let tmp = line.split('-').collect::<Vec<_>>();

            println!("{:#?}", line);
            let mut file: String = file.to_string();
            let logs = "log.txt".to_string();
            file.pop();
            println!("{:#?}", file);
            let path = "/.config/amx/vimrc/";
            let file = path.to_string() + &file;
            let logs = path.to_string() + &logs;
            let mut f = shellexpand::tilde("~").to_string();
            let mut p = shellexpand::tilde("~").to_string();
            let mut l = shellexpand::tilde("~").to_string();
            p.push_str(&path);
            f.push_str(&file);
            l.push_str(&logs);
            println!("{:#?}", f);
            println!("{:#?}", l);

            let check = Path::new(&f).exists();
            if check == false {
                println!("{}", "download vimrc");
                vimrc_file_dl();
            }

            let line = tmp.iter().nth(0);
            let end = tmp.iter().nth_back(0);
            println!("{:#?}", end);
            vimrc_file_read(f.to_string(), line.expect("REASON").to_string(), end.expect("REASON").to_string());
            let o = fs::read_to_string(&l).expect("could not read file");
            let st = "<pre><code>";
            let ed = "</code></pre>";
            let oo =  st.to_owned() + &o.to_string() + &ed;
            //let content = RoomMessageEventContent::text_markdown(&o);
            let content = RoomMessageEventContent::text_html(&o, &oo);
            room.send(content, None).await.unwrap();
        }
    }
    if text_content.body.contains("!rm vimrc") {
        println!("{}", "rm vimrc");
        vimrc_file_rm();
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

    if let Ok(text) = c.string_flag("type") {
        amx_setting_user(c);
        let status = &*text.to_string();
            match &*status {
                "vimrc" => client.add_event_handler(amx_timeline_bot_vimrc),
                "test" => client.add_event_handler(amx_timeline_bot_test),
                _ => client.add_event_handler(amx_timeline),
            };
    } else {
        client.add_event_handler(amx_timeline);
    };
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
    } else if let Ok(_room) = c.string_flag("room") {
        amx_setting_user(c);
        amx_setting_room(c);
        let data = Datasm::new().unwrap();
        let id: &str = &data.room;
        let room_id = <&RoomId>::try_from(id).unwrap();
        if let Some(room) = client.get_joined_room(&room_id) {
            room.send(content, None).await?;
        }
    } else {
        let data = Datasm::new().unwrap();
        let id: &str = &data.room;
        let room_id = <&RoomId>::try_from(id).unwrap();
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
