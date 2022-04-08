use hyper::Uri;
use serde::Deserialize;

use crate::db;
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
    pub clicks: i64,
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

        match form_data.url.parse::<Uri>() {
            Ok(uri) => {
                let scheme = match uri.scheme_str() {
                    Some(scheme) => scheme,
                    None => return Err(Error::InvalidURLFormat),
                };

                if !scheme.eq("http") && !scheme.eq("https") {
                    return Err(Error::InvalidURLFormat);
                }

                let host = match uri.host() {
                    Some(host) => host,
                    None => return Err(Error::InvalidURLFormat),
                };

                let path = form_data
                    .url
                    .strip_prefix(&format!("{}://{}", scheme, host));

                let path = match path {
                    Some(path) => path,
                    None => return Err(Error::InvalidURLFormat),
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
pub async fn url_stats(handle: &db::Handle, identifier: String) -> Result<UrlStat, Error> {
    let client = handle.client().await?;
    let data = client
        .query("SELECT * FROM app.get_url_stats($1)", &[&identifier])
        .await?;

    // TODO: Use a data mapper
    let db_id = data[0].try_get(0)?;
    let db_clicks = data[0].try_get(1)?;
    let db_url = data[0].try_get(2)?;

    Ok(UrlStat {
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
