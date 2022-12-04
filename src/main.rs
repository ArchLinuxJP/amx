pub mod data;
use data::Data as Datas;
use seahorse::{App, Command, Context, Flag, FlagType};
//use serde::{Deserialize, Serialize};
use std::env;
use url::Url;
use matrix_sdk::{
    Client, Session,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("aroom [option] [x]")
        .command(
            Command::new("timeline")
            .usage("aroom t")
            .description("timeline")
            .alias("t")
            .action(t)
            .flag(
                Flag::new("room", FlagType::String)
                .description("user flag(ex: $ aroom t -r ArchLinuxJP_general:gitter.im)")
                .alias("r"),
                )
            )
        ;
    app.run(args);
}

#[tokio::main]
async fn token() -> matrix_sdk::Result<()> {
  let data = Datas::new().unwrap();
    let homeserver = Url::parse(&data.home_server)?;
    let client = Client::new(homeserver).await?;
    let user = &data.username;
    let response = client
        .login_username(user, &data.password)
        .initial_device_display_name("amr")
        .send()
        .await?;
    let session = Session {
        access_token: response.access_token,
        refresh_token: None,
        user_id: response.user_id,
        device_id: response.device_id,
    };
    client.restore_login(session).await?;
    Ok(())
}

fn t(_c: &Context) {
    let client = token().unwrap();
    println!("{:#?}", client);
}
