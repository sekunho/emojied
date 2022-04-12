use hyper::Uri;
use serde::Deserialize;

use crate::db;
use crate::emoji;
use crate::leaderboard;

#[derive(Debug, PartialEq)]
struct DbLink {
    identifier: String,
    scheme: String,
    host: String,
    path: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUrl {
    pub url: String,
    pub identifier: String,
}

#[derive(Debug)]
pub enum Error {
    DbConnectionFailed(db::Error),
    DbCommunicationFailed,
    FailedToPrepareQuery,
    EmptyColumn,
    DuplicateIdentifier,
    InvalidURLFormat,
    MissingScheme,
    UnsupportedScheme,
    MissingHost,
    MissingPath,
    InvalidIdentifier,
}

impl From<db::Error> for Error {
    fn from(e: db::Error) -> Self {
        Error::DbConnectionFailed(e)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(_: tokio_postgres::Error) -> Self {
        Error::DbCommunicationFailed
    }
}

// TODO: Rename it to something else. This name sucks!!
impl DbLink {
    pub fn new(mut form_data: CreateUrl) -> Result<DbLink, Error> {
        form_data.identifier = form_data.identifier.trim().to_string();

        form_data.identifier = if form_data.identifier.is_empty() {
            // TODO: ^ Parse it to a domain type to avoid needless validation Generate for them
            let emojis = emoji::random_sequence();
            println!("Emoji ID not provided. Generated: {}", emojis);

            emojis
        } else if emoji::is_valid(&form_data.identifier) {
            println!("{} is valid", &form_data.identifier);
            form_data.identifier
        } else {
            return Err(Error::InvalidIdentifier);
        };

        // TODO: Would be better to assume that CreateUrl is already lowercased
        form_data.url = normalize_scheme(form_data.url)?;

        match form_data.url.to_lowercase().parse::<Uri>() {
            Ok(uri) => {
                let scheme = match uri.scheme_str() {
                    Some(scheme) => scheme.to_lowercase(),
                    None => return Err(Error::MissingScheme),
                };

                if !scheme.eq("http") && !scheme.eq("https") {
                    return Err(Error::UnsupportedScheme);
                }

                let host = match uri.host() {
                    Some(host) => host,
                    None => return Err(Error::MissingHost),
                };

                let path = form_data
                    .url
                    .strip_prefix(&format!("{}://{}", scheme, host));

                let path = match path {
                    Some(path) => path,
                    None => return Err(Error::MissingPath),
                };

                Ok(DbLink {
                    identifier: form_data.identifier,
                    scheme: scheme.to_string(),
                    host: host.to_string(),
                    path: path.to_string(),
                })
            }
            Err(_e) => Err(Error::InvalidURLFormat),
        }
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
pub async fn insert_url(handle: &db::Handle, data: CreateUrl) -> Result<String, Error> {
    println!("Attempting to insert: {:#?}", data);

    let client = handle.client().await?;
    let link = DbLink::new(data)?;

    println!("Inserting...");
    let row = client
        .query_one(
            "SELECT app.insert_url($1, $2, $3, $4)",
            &[&link.identifier, &link.scheme, &link.host, &link.path],
        )
        .await;

    println!("Attempt complete.");
    // TODO: Handle error properly
    match row {
        Ok(row) => match row.try_get(0) {
            Ok(url) => Ok(url),
            Err(_) => Err(Error::EmptyColumn),
        },
        Err(_e) => Err(Error::DuplicateIdentifier),
    }
}

// TODO: Move this to own module
/// # Errors
///
/// Will return `Err` when it fails to communicate with the DB.
pub async fn url_stats(
    handle: &db::Handle,
    identifier: String
) -> Result<leaderboard::Entry, Error> {
    let client = handle.client().await?;
    let data = client
        .query("SELECT * FROM app.get_url_stats($1)", &[&identifier])
        .await?;

    // TODO: Use a data mapper
    let db_id = data[0].try_get(0)?;
    let db_clicks = data[0].try_get(1)?;
    let db_url = data[0].try_get(2)?;

    Ok(leaderboard::Entry {
        identifier: db_id,
        clicks: db_clicks,
        url: db_url,
    })
}

/// # Errors
///
/// Will return `Err` when:
///
/// 1) `get_url` is NULL, which tbh is not possible but it's a function.
/// 2) it runs into a problem in communicating with the DB.
pub async fn fetch_url(handle: &db::Handle, identifier: String) -> Result<String, Error> {
    let client = handle.client().await?;
    let row = client
        .query_one("SELECT app.get_url($1)", &[&identifier])
        .await?;

    row.try_get(0).map_err(|e| Error::from(e))
}

/// Normalizes the scheme part of a URI
fn normalize_scheme(url: String) -> Result<String, Error> {
    let mut split_url = url.split("://");

    // If it doesn't exist then it's not a valid URL anyway.
    let scheme = match split_url.next() {
        Some(scheme) => scheme.to_lowercase(),
        None => return Err(Error::InvalidURLFormat)
    };

    let url = split_url
        .fold(scheme, |mut acc, chunk| {
            acc.push_str("://");
            acc.push_str(chunk);
            acc
        });

    Ok(url)
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
    fn normalizes_scheme() {
        let uri = "hTTPs://sekun.dev".to_string();

        assert_eq!(
            normalize_scheme(uri).unwrap(),
            "https://sekun.dev".to_string()
        );
    }

    #[test]
    fn ignores_subsequent_scheme_matches() {
        let uri = "hTTPs://sekun.dev/hTtps://foobar.com".to_string();

        assert_eq!(
            normalize_scheme(uri).unwrap(),
            "https://sekun.dev/hTtps://foobar.com"
        )
    }

    #[test]
    fn split_protocol_from_url() {
        let url = "https://sekun.dev";
        let split_url: Vec<&str> = url.split("://").collect();

        assert_eq!(split_url[0], "https");
        assert_eq!(split_url[1], "sekun.dev");
    }
}
