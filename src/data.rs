use config::{Config, ConfigError, File};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;


#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct Data {
    pub home_server: Cow<'static, str>,
    pub username: Cow<'static, str>,
    pub password: Cow<'static, str>,
}

impl Data {
    pub fn new() -> Result<Self, ConfigError> {
        let d = shellexpand::tilde("~") + "/.config/amx/config.toml";
        let s = Config::builder()
            .add_source(File::with_name(&d))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}

