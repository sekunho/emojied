use std::{env, num, path::PathBuf};

use thiserror::Error;

pub struct AppConfig {
    /// Application host
    pub host: String,

    // Port `emojied` will run on
    pub port: u16,

    /// Pool size
    pub database: su_sqlite::config::Config,
    pub static_assets_path: PathBuf,
}

#[derive(Debug, Error)]
pub enum CreateConfigError {
    #[error("var error")]
    VarError(#[from] env::VarError),
    #[error("var invalid format")]
    InvalidVarFormat(#[from] num::ParseIntError),
    #[error("missing static assets path")]
    MissingStaticAssetsPath,
    #[error("{0}")]
    DB(#[from] config::ConfigError),
}

impl AppConfig {
    pub fn from_env() -> Result<AppConfig, CreateConfigError> {
        let app_port = match env::var("APP_SERVER__PORT") {
            Ok(port) => port.parse::<u16>()?,
            Err(_) => 3000,
        };

        let static_assets_path = env::var("APP__SERVER__STATIC_ASSETS")
            .map_err(|_| CreateConfigError::MissingStaticAssetsPath)?;

        let static_assets_path = PathBuf::from(static_assets_path);

        tracing::info!("Static assets: {:?}", static_assets_path);

        let database = su_sqlite::config::Config::from_env("APP__DATABASE", "__")
            .inspect_err(|err| tracing::error!("error: {}", err))?;

        Ok(AppConfig {
            host: "emojied.net".to_string(),
            port: app_port,
            database,
            static_assets_path,
        })
    }
}
