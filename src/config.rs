use deadpool_postgres::{ManagerConfig, RecyclingMethod};
use tokio_postgres::config::SslMode;
use std::env;

pub struct AppConfig {
    /// Application host
    pub host: String,
    /// PostgreSQL config
    pub pg: tokio_postgres::Config,
    /// Pool manager config
    pub manager: ManagerConfig,
    /// Pool size
    pub pool_size: usize,
    pub ca_cert_path: Option<String>
}

#[derive(Debug)]
pub enum Error {
    VarError(String, env::VarError),
    InvalidVarFormat,
}

impl AppConfig {
    pub fn from_env() -> Result<AppConfig, Error> {
        let mut pg_config = tokio_postgres::Config::new();
        let manager_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };

        // NOTE: Is there a better way than matching & manually early returning?

        let host = match env::var("PG__HOST") {
            Ok(host) => host,
            Err(e) => return Err(Error::VarError("PG__HOST".to_string(), e)),
        };

        let user = match env::var("PG__USER") {
            Ok(user) => user,
            Err(e) => return Err(Error::VarError("PG__USER".to_string(), e)),
        };

        let dbname = match env::var("PG__DBNAME") {
            Ok(dbname) => dbname,
            Err(e) => return Err(Error::VarError("PG__DBNAME".to_string(), e)),
        };

        let dbpassword = match env::var("PG__PASSWORD") {
            Ok(dbpassword) => dbpassword,
            Err(e) => return Err(Error::VarError("PG__PASSWORD".to_string(), e))
        };

        let port = match env::var("PG__PORT") {
            Ok(port) => match port.parse::<u16>() {
                Ok(port) => port,
                Err(_) => return Err(Error::InvalidVarFormat),
            },
            Err(_e) => 5432,
        };

        let pool_size = match env::var("PG__POOL_SIZE") {
            Ok(pool_size) => match pool_size.parse::<usize>() {
                Ok(pool_size) => pool_size,
                Err(_) => return Err(Error::InvalidVarFormat),
            },
            Err(_e) => 22,
        };

        let ca_cert_path = match env::var("PG__CA_CERT") {
            Ok(path) => Some(path),
            Err(_e) => None,
        };

        pg_config.application_name("emojied");
        pg_config.host(&host);
        pg_config.password(&dbpassword);
        pg_config.user(&user);
        pg_config.dbname(&dbname);
        pg_config.port(port);

        // TODO: Set only when CA CERT is present
        pg_config.ssl_mode(SslMode::Require);

        Ok(AppConfig {
            host: "emojied.net".to_string(),
            pg: pg_config,
            manager: manager_config,
            pool_size,
            ca_cert_path,
        })
    }
}
