use std::env;

pub struct AppConfig {
    pub database_url: String,
    pub host: String,
}

impl AppConfig {
    pub fn new() -> Result<AppConfig, env::VarError>{
        let database_url = env::var("DATABASE_URL")?;
        let host = env::var("APP_HOST")?;

        Ok(AppConfig { database_url, host })
    }
}
