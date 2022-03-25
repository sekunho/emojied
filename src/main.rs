#![forbid(unsafe_code)]

use emojiurl::db::DbHandle;
use std::process;

#[tokio::main]
async fn main() {
    match DbHandle::new().await {
        Ok(db_handle) => {
            // https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
            if let Err(e) = emojiurl::run(db_handle).await {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }

        Err(e) => {
            eprintln!("OH NO: {}", e);
            process::exit(1);
        }
    };
}
