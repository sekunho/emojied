use deadpool_postgres::{ManagerConfig, RecyclingMethod};
use std::{env, error, fmt, num, path::PathBuf};
use tokio_postgres::config::SslMode;

#[derive(Clone)]
pub struct AppConfig {
    /// Application host
    pub host: String,
    /// PostgreSQL config
    pub pg: tokio_postgres::Config,

    // Port `emojied` will run on
    pub port: u16,

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
    InvalidDBPasswordFile,
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
            },

            Error::InvalidVarFormat(error) => {
                write!(f, "{}: Unable to parse value to integer", error)
            }

            Error::MissingStaticAssetsPath => {
                write!(
                    f,
                    "Missing environment variable: `APP__STATIC_ASSETS=<PATH_TO_FILES>`"
                )
            }
            Error::InvalidDBPasswordFile => {
                write!(f, "Unable to read DB password file")
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
        let app_port = match env::var("APP__PORT") {
            Ok(port) => port.parse::<u16>()?,
            Err(_) => 3000,
        };

        let static_assets_path =
            env::var("APP__STATIC_ASSETS").map_err(|_| Error::MissingStaticAssetsPath)?;

        let static_assets_path = PathBuf::from(static_assets_path);

        println!("Static assets: {:?}", static_assets_path);

        let mut pg_config = tokio_postgres::Config::new();
        let manager_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };

        let host = env::var("PG__HOST")?;
        println!("Database Host: {}", host);

        let user = env::var("PG__USER")?;
        println!("Database User: {}", user);

        let dbname = env::var("PG__DBNAME")?;
        println!("Database Name: {}", dbname);

        match env::var("PG__PASSWORD") {
            Ok(dbpassword) => {
                println!("Database Password: [REDACTED]");
                pg_config.password(&dbpassword);
            }
            Err(_) => match env::var("PG__PASSWORD_FILE") {
                Ok(dbpassword_file) => {
                    println!("Database Password File: {}", dbpassword_file);

                    let dbpassword =
                        std::fs::read(dbpassword_file).map_err(|_| Error::InvalidDBPasswordFile)?;

                    let dbpassword: String = String::from_utf8(dbpassword)
                        .map_err(|_| Error::InvalidDBPasswordFile)?
                        .strip_suffix("\n")
                        .ok_or(Error::InvalidDBPasswordFile)?
                        .to_string();

                    pg_config.password(&dbpassword);
                }

                Err(_) => (),
            },
        }

        let port = match env::var("PG__PORT") {
            Ok(port) => port.parse::<u16>()?,
            Err(_e) => 5432,
        };

        println!("Database Port: {}", port);

        let pool_size = match env::var("PG__POOL_SIZE") {
            Ok(pool_size) => pool_size.parse::<usize>()?,
            Err(_e) => 22,
        };

        println!("Database Pool Size: {}", pool_size);

        // Not providing CA_CERT is fine
        let ca_cert_path = match env::var("PG__CA_CERT") {
            Ok(path) => {
                pg_config.ssl_mode(SslMode::Require);
                println!("Database CA Certificate Path: {}", path);
                Some(path)
            }
            Err(_e) => None,
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
            port: app_port,
            manager: manager_config,
            pool_size,
            ca_cert_path,
            static_assets_path,
        })
    }
}
