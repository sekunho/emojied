use std::env;
use deadpool_postgres::{ManagerConfig, RecyclingMethod};

pub struct Config {
    pub host: String,
    pub pg: tokio_postgres::Config,
    pub manager: ManagerConfig,
    pub pool_size: usize,
}

#[derive(Debug)]
pub enum Error {
    VarError(env::VarError),
}

impl Config {
    pub fn from_env() -> Result<Config, Error>{
        let mut pg_config = tokio_postgres::Config::new();
        let manager_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast
        };

        // TODO: Get from environment instead
        pg_config.host("localhost");
        pg_config.user("postgres");
        pg_config.dbname("emojied_db");

        Ok(Config {
            host: "emojied.net".to_string(),
            pg: pg_config,
            manager: manager_config,
            pool_size: 22,
        })
    }
}
