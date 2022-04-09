use crate::db;

#[derive(Debug)]
pub struct Entry {
    pub identifier: String,
    pub url: String,
    pub clicks: i64
}

pub enum Error {
    DbConnectionFailed(db::Error),
    DbCommunicationFailed,
    FailedToPrepareQuery,
    EmptyColumn,
}

impl From<db::Error> for Error {
    fn from(e: db::Error) -> Self {
        Error::DbConnectionFailed(e)
    }
}

pub async fn fetch(handle: &db::Handle) -> Result<Vec<Entry>, Error> {
    let client = handle.client().await?;

    let entries = client
        .query(
            "SELECT * FROM app.leaderboard()",
            &[]
        )
        .await
        .map_err(|_| Error::DbCommunicationFailed)?;

    entries
        .into_iter()
        .map(|row| -> Result<Entry, Error> {
            let identifier = row
                .try_get(0)
                .map_err(|_| Error::EmptyColumn)?;

            let clicks = row
                .try_get(1)
                .map_err(|_| Error::EmptyColumn )?;

            let url = row
                .try_get(2)
                .map_err(|_| Error::EmptyColumn )?;

            Ok(Entry { identifier, url, clicks })
        })
        .collect()
}
