// TODO: Use a connection pool for the DB
// TODO: Get rid of `sqlx` for `tokio-postgres` + `deadpool-postgres` + `tokio-pg-mapper`
// NOTE: This module should probably just handle client creation, statement caching.
// TODO: Extract calls to BL to their own modules? Idk.

use deadpool_postgres::{Manager, Object, Pool};
use hyper::http::Uri;
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

// TLS
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;

use crate::config::AppConfig;
use crate::emoji;

pub struct Handle {
    pub pool: Pool,
}

#[derive(Debug, PartialEq)]
struct DbLink {
    identifier: String,
    scheme: String,
    host: String,
    path: String,
}

pub struct UrlStat {
    pub identifier: String,
    pub url: String,
    pub clicks: i64,
}

// TODO: Break this error into multiple ones. DB & business logic stuff?
#[derive(Serialize, Debug)]
pub enum Error {
    URIParseFailed,
    IdentifierParseFailed,
    NoRecord,
    DuplicateIdentifier,
    DbPooped,
    EmptyColumn,
    PoolError,
    FailedToPrepareQuery,
    MissingCACertFile,
    InvalidCertificateFormat,
    FailedToBuildTlsConnector,
    FailedToBuildPool,
}

impl DbLink { pub fn new(mut form_data: CreateUrl) -> Option<DbLink> {
        form_data.identifier = form_data.identifier.trim().to_string();

        form_data.identifier = if form_data.identifier.is_empty() {
            // TODO: ^ Parse it to a domain type to avoid needless validation
            // Generate for them
            emoji::new_emoji_id()
        } else if emoji::is_valid(&form_data.identifier) {
            form_data.identifier
        } else {
            return None;
        };

        match form_data.url.parse::<Uri>() {
            Ok(uri) => {
                let scheme = uri.scheme_str()?;
                let host = uri.host()?;
                let path = form_data
                    .url
                    .strip_prefix(&format!("{}://{}", scheme, host))?;

                Some(DbLink {
                    identifier: form_data.identifier,
                    scheme: scheme.to_string(),
                    host: host.to_string(),
                    path: path.to_string(),
                })
            }
            Err(_e) => {
                // TODO: Actually handle errors? Idk.
                None
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateUrl {
    pub url: String,
    pub identifier: String,
}

impl Handle {
    pub async fn new(app_config: AppConfig) -> Result<Handle, Error> {
        let manager = match app_config.ca_cert_path {
            Some(ca_cert_path) => {
                let cert = std::fs::read(ca_cert_path)
                    .map_err(|_| Error::MissingCACertFile)?;

                let ntls_cert = Certificate::from_pem(&cert)
                    .map_err(|_| Error::InvalidCertificateFormat)?;

                let tls = TlsConnector::builder()
                    .add_root_certificate(ntls_cert)
                    .build()
                    .map_err(|_| Error::FailedToBuildTlsConnector)?;

                let conn = MakeTlsConnector::new(tls);

                Manager::from_config(app_config.pg, conn, app_config.manager)
            },
            None => Manager::from_config(app_config.pg, NoTls, app_config.manager),
        };

        let pool = Pool::builder(manager)
            .max_size(app_config.pool_size)
            .build()
            .map_err(|_| Error::FailedToBuildPool)?;

        Ok(Handle { pool })
    }

    /// Creates a new client in a pool
    pub async fn client(&self) -> Result<Object, Error> {
        match self.pool.get().await {
            Ok(client) => Ok(client),
            Err(e) => {
                eprintln!("Pool error: {}", e);
                Err(Error::PoolError)
            },
        }
    }

    // TODO: Move this to own module
    /// Inserts the URL to be shortened in the DB.
    ///
    /// # Errors
    ///
    /// Will return `Err` when:
    ///
    /// 1) The insert fails due to duplicate identifiers
    /// 2) Parsing of the URI fails
    pub async fn insert_url(&self, data: CreateUrl) -> Result<String, Error> {
        match DbLink::new(data) {
            Some(link) => {
                let client = self.client().await?;

                let stmt = match client
                    .prepare_cached("SELECT app.insert_url($1, $2, $3, $4)")
                    .await
                {
                    Ok(stmt) => stmt,
                    Err(_) => return Err(Error::FailedToPrepareQuery),
                };

                let row = client
                    .query_one(
                        &stmt,
                        &[&link.identifier, &link.scheme, &link.host, &link.path],
                    )
                    .await;

                match row {
                    Ok(row) => match row.try_get(0) {
                        Ok(url) => Ok(url),
                        Err(_) => Err(Error::EmptyColumn),
                    },
                    Err(_e) => Err(Error::DuplicateIdentifier),
                }
            }
            None => Err(Error::URIParseFailed),
        }
    }

    // TODO: Move this to own module
    /// # Errors
    ///
    /// Will return `Err` when it fails to communicate with the DB.
    pub async fn url_stats(&self, identifier: String) -> Result<UrlStat, Error> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(_e) => {
                return Err(Error::PoolError);
            }
        };

        let stmt = match client
            .prepare_cached("SELECT * FROM app.get_url_stats($1)")
            .await
        {
            Ok(stmt) => stmt,
            Err(_e) => {
                return Err(Error::FailedToPrepareQuery);
            }
        };

        let data = client.query_one(&stmt, &[&identifier]).await;

        match data {
            Ok(data) => {
                // Dear god this is painful
                let db_id = match data.try_get(0) {
                    Ok(db_id) => db_id,
                    Err(_) => {
                        return Err(Error::EmptyColumn);
                    }
                };

                let db_clicks = match data.try_get(1) {
                    Ok(db_clicks) => db_clicks,
                    Err(_) => {
                        return Err(Error::EmptyColumn);
                    }
                };

                let db_url = match data.try_get(2) {
                    Ok(db_url) => db_url,
                    Err(_) => {
                        return Err(Error::EmptyColumn);
                    }
                };

                Ok(UrlStat {
                    identifier: db_id,
                    clicks: db_clicks,
                    url: db_url,
                })
            }
            Err(_) => Err(Error::DbPooped),
        }
    }

    // TODO: Move this to own module
    /// # Errors
    ///
    /// Will return `Err` when:
    ///
    /// 1) `get_url` is NULL, which tbh is not possible but it's a function.
    /// 2) it runs into a problem in communicating with the DB.
    pub async fn fetch_url(&self, identifier: String) -> Result<String, Error> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(_e) => {
                return Err(Error::PoolError);
            }
        };

        let stmt = match client.prepare_cached("SELECT app.get_url($1)").await {
            Ok(stmt) => stmt,
            Err(_) => {
                return Err(Error::FailedToPrepareQuery);
            }
        };

        let rows = client.query(&stmt, &[&identifier]).await;

        match rows {
            Ok(rows) => {
                // I don't think it's really a NoRecord. More like NULL col?
                match rows[0].try_get(0) {
                    Ok(url) => Ok(url),
                    Err(_) => Err(Error::EmptyColumn),
                }
            }
            Err(_e) => Err(Error::DbPooped),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_without_path() {
        let data = CreateUrl {
            url: "https://news.ycombinator.com".to_string(),
            identifier: "🍊🌐".to_string(),
        };

        assert_eq!(
            DbLink::new(data).unwrap(),
            DbLink {
                identifier: "🍊🌐".to_string(),
                scheme: "https".to_string(),
                host: "news.ycombinator.com".to_string(),
                path: "".to_string()
            }
        )
    }

    #[test]
    fn valid_emoji_range() {
        assert!(emoji::emoji_range().iter().all(|e| unic_emoji_char::is_emoji(e)))
    }
}
