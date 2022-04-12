use deadpool_postgres::{ManagerConfig, RecyclingMethod};
use std::{env, error, fmt, num, path::PathBuf};
use tokio_postgres::config::SslMode;

#[derive(Clone)]
pub struct AppConfig {
    /// Application host
    pub host: String,
    /// PostgreSQL config
    pub pg: tokio_postgres::Config,
    /// Pool manager config
    pub manager: ManagerConfig,
    /// Pool size
    pub pool_size: usize,
    pub ca_cert_path: Option<String>,
    pub static_assets_path: PathBuf,
}

#[derive(Debug)]
pub enum Error {
    VarError(env::VarError),
    InvalidVarFormat(num::ParseIntError),
    MissingStaticAssetsPath,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::VarError(var_error) => match var_error {
                env::VarError::NotPresent => {
                    write!(f, "An environment variable is required but is not present")
                }
                env::VarError::NotUnicode(_) => {
                    write!(
                        f,
                        "An environment variable is expected to be unicode, but isn't"
                    )
                }
            }

            Error::InvalidVarFormat(error) => {
                write!(f, "{}: Unable to parse value to integer", error)
            }

            Error::MissingStaticAssetsPath => {
                write!(f, "Missing environment variable: `APP__STATIC_ASSETS=<PATH_TO_FILES>`")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<env::VarError> for Error {
    fn from(ve: env::VarError) -> Self {
        Error::VarError(ve)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(ne: num::ParseIntError) -> Self {
        Error::InvalidVarFormat(ne)
    }
}

impl AppConfig {
    pub fn from_env() -> Result<AppConfig, Error> {
        let static_assets_path = env::var("APP__STATIC_ASSETS")
            .map_err(|_| Error::MissingStaticAssetsPath)?;
        let static_assets_path = PathBuf::from(static_assets_path);

        println!("Static assets: {:?}", static_assets_path);

        let mut pg_config = tokio_postgres::Config::new();
        let manager_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };

        let host = env::var("PG__HOST")?;
        let user = env::var("PG__USER")?;
        let dbname = env::var("PG__DBNAME")?;

        match env::var("PG__PASSWORD") {
            Ok(dbpassword) => {
                pg_config.password(&dbpassword);
            },
            Err(_) => (),
        }

        let port = match env::var("PG__PORT") {
            Ok(port) => port.parse::<u16>()?,
            Err(_e) => 5432,
        };

        let pool_size = match env::var("PG__POOL_SIZE") {
            Ok(pool_size) => pool_size.parse::<usize>()?,
            Err(_e) => 22,
        };

        // Not providing CA_CERT is fine
        let ca_cert_path = match env::var("PG__CA_CERT") {
            Ok(path) => {
                pg_config.ssl_mode(SslMode::Require);
                Some(path)
            },
            Err(_e) => {
                None
            }
        };

        pg_config
            .application_name("emojied")
            .host(&host)
            .user(&user)
            .dbname(&dbname)
            .port(port);

        Ok(AppConfig {
            host: "emojied.net".to_string(),
            pg: pg_config,
            manager: manager_config,
            pool_size,
            ca_cert_path,
            static_assets_path,
        })
    }
}
