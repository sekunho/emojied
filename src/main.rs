#![forbid(unsafe_code)]

use emojied::db;
use std::env;
use std::process;

use emojied::config::Config;

// TODO: Read env vars for config

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap_or_else(|err| {
        eprintln!("Application config error: {:?}", err);
        process::exit(1);
    });

    match db::Handle::new(config).await {
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
