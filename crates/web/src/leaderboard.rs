use deadpool_sqlite::rusqlite;
use thiserror::Error;

use crate::sql;

#[derive(Debug)]
pub struct Entry {
    pub identifier: String,
    pub url: String,
    pub clicks: i64,
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("failed to acquire a connection from the pool. reason: {0}")]
    Pool(#[from] deadpool_sqlite::PoolError),
    #[error("failed to fetch leaderboard entries. reason: {0}")]
    Interact(#[from] deadpool_sqlite::InteractError),
    #[error("query failed. reason: {0}")]
    DB(#[from] deadpool_sqlite::rusqlite::Error),
}

pub async fn fetch(db_handle: &su_sqlite::handle::Handle) -> Result<Vec<Entry>, FetchError> {
    let conn = db_handle.get_read_conn().await?;

    let entries: Vec<Entry> = conn
        .interact(|conn| -> Result<Vec<Entry>, rusqlite::Error> {
            let mut statement = conn.prepare(sql::SELECT_LEADERBOARD)?;

            statement
                .query_map([], |row| -> Result<Entry, _> {
                    let short_name = row.get("short_name")?;
                    let target_url = row.get("target_url")?;
                    let clicks = row.get("clicks")?;

                    Ok(Entry {
                        identifier: short_name,
                        url: target_url,
                        clicks,
                    })
                })?
                .collect::<Result<Vec<Entry>, _>>();

            todo!()
        })
        .await??;

    Ok(entries)
}
