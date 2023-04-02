use crate::config::AppConfig;
use deadpool_postgres::{Manager, Object, Pool, PoolError};
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::{fmt, io};
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct Handle {
    pub pool: Pool,
}

// TODO: Break this error into multiple ones. DB & business logic stuff?
#[derive(Debug)]
pub enum Error {
    PoolError(PoolError),
    CACertFileError(io::Error),
    InvalidCACert,
    FailedToBuildTlsConnector,
    FailedToBuildPool,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PoolError(pe) => write!(f, "Couldn't get a client from the pool: {}", pe),
            Error::CACertFileError(ie) => write!(f, "Failed to read CA cert: {}", ie),
            Error::InvalidCACert => write!(f, "Invalid PEM file"),
            Error::FailedToBuildTlsConnector => {
                write!(f, "Couldn't build a TLS connector with this certificate")
            }
            Error::FailedToBuildPool => write!(f, "Failed to build a database pool"),
        }
    }
}

impl From<PoolError> for Error {
    fn from(pe: PoolError) -> Self {
        Error::PoolError(pe)
    }
}

impl Handle {
    pub async fn new(app_config: AppConfig) -> Result<Handle, Error> {
        let manager = match app_config.ca_cert_path {
            Some(ca_cert_path) => {
                let cert = std::fs::read(ca_cert_path).map_err(|e| Error::CACertFileError(e))?;

                let ntls_cert = Certificate::from_pem(&cert).map_err(|_| Error::InvalidCACert)?;

                let tls = TlsConnector::builder()
                    .add_root_certificate(ntls_cert)
                    .build()
                    .map_err(|_| Error::FailedToBuildTlsConnector)?;

                let conn = MakeTlsConnector::new(tls);

                Manager::from_config(app_config.pg, conn, app_config.manager)
            }
            None => Manager::from_config(app_config.pg, NoTls, app_config.manager),
        };

        let pool = Pool::builder(manager)
            .max_size(app_config.pool_size)
            .build()
            .map_err(|_| Error::FailedToBuildPool)?;

        Ok(Handle { pool })
    }

    /// Creates a new client in a pool.
    ///
    /// You'll need a client to execute queries. Can't use the pool directly.
    pub async fn client(&self) -> Result<Object, Error> {
        println!("Getting client from pool...");
        self.pool.get().await.map_err(|e| Error::from(e))
    }
}
