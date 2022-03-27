// TODO: Use a connection pool for the DB

use hyper::http::Uri;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, NoTls};

pub struct DbHandle {
    pub client: Client,
}

#[derive(Debug, PartialEq)]
struct DbLink {
    identifier: String,
    scheme: String,
    host: String,
    path: String,
}

#[derive(Serialize)]
pub enum InsertError {
    ParseFailed,
    DuplicateIdentifier,
}

impl DbLink {
    pub fn new(form_data: CreateUrl) -> Option<DbLink> {
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
            Err(e) => {
                println!("{}", e);
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

impl DbHandle {
    pub async fn new() -> Result<DbHandle, Error> {
        let (client, connection) =
            tokio_postgres::connect("host=localhost user=postgres dbname=emojiurl_db", NoTls)
                .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error: {}", e);
            }
        });

        Ok(DbHandle { client })
    }

    /// Inserts the URL to be shortened in the DB.
    pub async fn insert_url(&self, data: CreateUrl) -> Result<String, InsertError> {
        // TODO: Generate random string of emojis
        // TODO: Optionally get emojis from user, and validate it.

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
                let rows = self
                    .client
                    .query(
                        "SELECT app.insert_url($1, $2, $3, $4)",
                        &[&link.identifier, &link.scheme, &link.host, &link.path],
                    )
                    .await;

                match rows {
                    Ok(_) => Ok(link.identifier),
                    Err(e) => {
                        // TODO: Read docs if I can pattern match the data
                        // constructors
                        println!("{}", e);
                        Err(InsertError::DuplicateIdentifier)
                    }
                }
            }
            None => Err(InsertError::ParseFailed),
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
}
