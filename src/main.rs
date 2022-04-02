#![forbid(unsafe_code)]

use emojied::db;
use std::process;

// TODO: Read env vars for config

#[tokio::main]
async fn main() {
    match db::Handle::new().await {
        Ok(db_handle) => {
            // https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
            if let Err(e) = emojied::run(db_handle).await {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }

        Err(e) => {
            eprintln!("Database error: {}", e);
            process::exit(1);
        }
    };
}
