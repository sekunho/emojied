// TODO: Use a connection pool for the DB

use hyper::http::Uri;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};
use tiny_id::ShortCodeGenerator;
use unic_char_range::{chars, CharRange};
use crate::emoji;

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
pub enum DbError {
    URIParseFailed,
    IdentifierParseFailed,
    NoRecord,
    DuplicateIdentifier,
    SQLXError,
}

impl DbLink {
    pub fn new(mut form_data: CreateUrl) -> Option<DbLink> { form_data.identifier = form_data.identifier.trim().to_string();

        form_data.identifier =
            if form_data.identifier == "" {
                // TODO: ^ Parse it to a domain type to avoid needless validation
                // Generate for them
                new_emoji_id()
            } else {
                if emoji::is_emoji(&form_data.identifier) {
                    form_data.identifier
                } else {
                    return None;
                }
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

pub struct DbHandle {
    pub pool: Pool<Postgres>,
}

impl DbHandle {
    pub async fn new() -> Result<DbHandle, sqlx::Error> {
        // TODO: Load database URL from `AppConfig`
        // Create a connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres@localhost/emojied_db")
            .await?;

        Ok(DbHandle { pool })
    }

    /// Inserts the URL to be shortened in the DB.
    pub async fn insert_url(&self, data: CreateUrl) -> Result<String, DbError> {
        match DbLink::new(data) {
            Some(link) => {
                // TODO: Refactor when tokio-postgres supports casting in func args.
                // e.g SELECT app.insert_url($1, $2::app.SCHEME, $3, $4, $5)
                // ^ doesn't work since it doesn't respect `::app.SCHEME`.
                //
                // ```
                // Err(Error { kind: ToSql(1), cause: Some(WrongType { postgres: Other(
                // Other { name: "scheme", oid: 256012, kind: Enum(["http",
                // "https"]), schema: "app" }), rust: "str" }) })
                // ```
                let rows = sqlx::query!(
                        "SELECT app.insert_url($1, $2, $3, $4)",
                        &link.identifier,
                        &link.scheme,
                        &link.host,
                        &link.path
                    )
                    .fetch_one(&self.pool)
                    .await;

                match rows {
                    Ok(_) => Ok(link.identifier),
                    Err(_e) => {
                        // TODO: Read docs if I can pattern match the data constructors
                        Err(DbError::DuplicateIdentifier)
                    }
                }
            }
            None => Err(DbError::URIParseFailed),
        }
    }

    pub async fn fetch_url(
        &self,
        identifier: String
        ) -> Result<String, DbError> {
        let row = sqlx::query!("SELECT app.get_url($1)", &identifier)
            .fetch_one(&self.pool)
            .await;

        match row {
            Ok(row) => {
                if let Some(url) = row.get_url {
                    Ok(url)
                } else {
                    Err(DbError::NoRecord)
                }
            },
            Err(_e) => Err(DbError::SQLXError)
        }
    }

    pub async fn url_stats(
        &self,
        identifier: String
    ) -> Result<UrlStat, DbError> {
        let row = sqlx::query!("SELECT * FROM app.get_url_stats($1)", &identifier)
            .fetch_one(&self.pool)
            .await;

        match row {
            Ok(row) => {
                // NOTE: Idk, the columns have NOT NULL constraints on them.
                let db_id = row.identifier.unwrap();
                let db_clicks = row.clicks.unwrap();
                let db_url = row.url.unwrap();

                Ok(UrlStat { identifier: db_id, clicks: db_clicks, url: db_url })
            }
            Err(_e) => Err(DbError::SQLXError)
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
