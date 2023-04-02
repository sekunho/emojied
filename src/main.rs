#![forbid(unsafe_code)]

use std::process;

use emojied::db;
use emojied::state::AppState;
use emojied::config::AppConfig;

#[tokio::main]
async fn main() {
    println!("Loading configuration from environment");
    let config = AppConfig::from_env().unwrap_or_else(|err| {
        eprintln!("Application config error: {}", err);
        process::exit(1);
    });

    println!("Attempting to establish a database connection");
    match db::Handle::new(config.clone()).await {
        Ok(db_handle) => {
            eprintln!("Database connection established");

            let state = AppState {
                config,
                db_handle,
            };

            // https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
            if let Err(e) = emojied::run(state).await {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }

        Err(e) => {
            eprintln!("Database error: {:#?}", e);
            process::exit(1);
        }
    };
}
