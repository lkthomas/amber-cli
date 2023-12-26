use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AmberConfig {
    pub base_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ApiToken {
    pub name: String,
    pub psk: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
    pub amberconfig: AmberConfig,
    pub userconfig: UserConfig,
    pub apitoken: ApiToken,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct UserConfig {
    pub state: String,
}

impl AppConfig {
    pub async fn get(app_config_file: String) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(&app_config_file))
            .build()?;

        config.try_deserialize()
    }
}
