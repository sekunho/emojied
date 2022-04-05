// TODO: Use a connection pool for the DB
// TODO: Get rid of `sqlx` for `tokio-postgres` + `deadpool-postgres` + `tokio-pg-mapper`
// NOTE: This module should probably just handle client creation, statement caching.
// TODO: Extract calls to BL to their own modules? Idk.

use hyper::http::Uri;
use serde::{Deserialize, Serialize};
use deadpool_postgres::{Pool, Manager, Object};
use tokio_postgres::NoTls;
use tiny_id::ShortCodeGenerator;
use unic_char_range::{chars, CharRange};

use crate::emoji;
use crate::config::Config;

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
    pub clicks: i64
}

#[derive(Serialize)]
pub enum Error {
    URIParseFailed,
    IdentifierParseFailed,
    NoRecord,
    DuplicateIdentifier,
    DbPooped,
    EmptyColumn,
    PoolError,
    FailedToPrepareQuery
}

impl DbLink {
    pub fn new(mut form_data: CreateUrl) -> Option<DbLink> {
        form_data.identifier = form_data.identifier.trim().to_string();

        form_data.identifier =
            if form_data.identifier.is_empty() {
                // TODO: ^ Parse it to a domain type to avoid needless validation
                // Generate for them
                new_emoji_id()
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
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateUrl {
    pub url: String,
    pub identifier: String,
}

impl Handle {
    pub async fn new(app_config: Config) -> Result<Handle, tokio_postgres::Error> {
        let manager = Manager::from_config(
            app_config.pg,
            NoTls,
            app_config.manager,
        );

        // TODO: Remove unwrap()
        let pool = Pool::builder(manager)
            .max_size(app_config.pool_size)
            .build()
            .unwrap();

        Ok(Handle { pool })
    }

    /// Creates a new client in a pool
    pub async fn client(&self) -> Result<Object, Error> {
        match self.pool.get().await {
            Ok(client) => Ok(client),
            Err(_) => Err(Error::PoolError)
        }
    }

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
                    .await {
                        Ok(stmt) => stmt,
                        Err(_) => { return Err(Error::FailedToPrepareQuery) },
                    };

                let row = client
                    .query_one(
                        &stmt,
                        &[&link.identifier, &link.scheme, &link.host, &link.path],
                    )
                    .await;

                match row {
                    Ok(row) => {
                        match row.try_get(0) {
                            Ok(url) => Ok(url),
                            Err(_) => Err(Error::EmptyColumn),
                        }
                    },
                    Err(_e) => Err(Error::DuplicateIdentifier)
                }
            }
            None => Err(Error::URIParseFailed),
        }
    }

    /// # Errors
    ///
    /// Will return `Err` when it fails to communicate with the DB.
    pub async fn url_stats(&self, identifier: String) -> Result<UrlStat, Error> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(_e) => { return Err(Error::PoolError); }
        };

        let stmt = match client
            .prepare_cached("SELECT * FROM app.get_url_stats($1)")
            .await {
                Ok(stmt) => stmt,
                Err(_e) => { return Err(Error::FailedToPrepareQuery); },
            };

        let data = client
            .query_one(&stmt, &[&identifier])
            .await;

        match data {
            Ok(data) => {
                // Dear god this is painful
                let db_id = match data.try_get(0) {
                    Ok(db_id) => db_id,
                    Err(_) => { return Err(Error::EmptyColumn); }
                };

                let db_clicks = match data.try_get(1) {
                    Ok(db_clicks) => db_clicks,
                    Err(_) => { return Err(Error::EmptyColumn); }
                };

                let db_url = match data.try_get(2) {
                    Ok(db_url) => db_url,
                    Err(_) => { return Err(Error::EmptyColumn); }
                };

                Ok(UrlStat { identifier: db_id, clicks: db_clicks, url: db_url })
            },
            Err(_) => Err(Error::DbPooped),
        }
    }

    /// # Errors
    ///
    /// Will return `Err` when:
    ///
    /// 1) `get_url` is NULL, which tbh is not possible but it's a function.
    /// 2) it runs into a problem in communicating with the DB.
    pub async fn fetch_url(
        &self,
        identifier: String
    ) -> Result<String, Error> {
        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(_e) => { return Err(Error::PoolError); },
        };

        let stmt = match client
            .prepare_cached("SELECT app.get_url($1)")
            .await {
                Ok(stmt) => stmt,
                Err(_) => { return Err(Error::FailedToPrepareQuery); },
            };

        let rows = client
            .query(&stmt, &[&identifier]).await;

        match rows {
            Ok(rows) => {
                // I don't think it's really a NoRecord. More like NULL col?
                match rows[0].try_get(0) {
                    Ok(url) => Ok(url),
                    Err(_) => Err(Error::EmptyColumn),
                }
            },
            Err(_e) => Err(Error::DbPooped)
        }
    }
}

fn new_emoji_id() -> String {
    // Sorry!
    // https://github.com/paulgb/tiny_id/blob/e15277384391524e043110bc0f8adadbc6f3c18d/README.md?plain=1#L93-L98=
    let emojis: Vec<char> = emoji_range().iter().collect();

    let mut gen =
        ShortCodeGenerator::with_alphabet(
            emojis,
            6
        );

    gen.next_string()
}

fn emoji_range() -> CharRange {
    // https://unicode.org/Public/emoji/14.0/emoji-sequences.txt
    chars!('\u{1f600}'..='\u{1f64f}')
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
        assert!(emoji_range().iter().all(|e| unic_emoji_char::is_emoji(e)))
    }
}
