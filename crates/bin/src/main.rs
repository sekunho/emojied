#![forbid(unsafe_code)]

use std::process;

use web::config::AppConfig;
use web::state::AppEnv;

#[tokio::main]
async fn main() {
    let fmt = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .with_thread_ids(true)
        .with_line_number(true);

    tracing::subscriber::set_global_default(fmt.json().finish()).unwrap();

    tracing::info!("Loading configuration from environment");
    let config = AppConfig::from_env().unwrap_or_else(|err| {
        tracing::error!("Application config error: {}", err);
        process::exit(1);
    });

    tracing::info!("Attempting to establish a database connection");
    match su_sqlite::handle::Handle::new(config.database.clone()) {
        Ok(db_handle) => {
            tracing::info!("Database connection established");

            let state = AppEnv { config, db_handle };

            // https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
            if let Err(e) = web::run(state).await {
                tracing::error!("Application error: {}", e);
                process::exit(1);
            }
        }

        Err(e) => {
            tracing::error!("Database error: {:#?}", e);
            process::exit(1);
        }
    };
}
