mod components;
pub mod config;
mod controllers;
mod emoji;
pub mod leaderboard;
mod sql;
pub mod state;
pub mod url;
mod views;

use axum::routing;
use axum::Router;
use state::AppEnv;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub async fn run(app_env: AppEnv) -> Result<(), std::io::Error> {
    let app_env = Arc::from(app_env);

    let app = Router::new()
        .fallback(controllers::not_found)
        .route("/", routing::get(controllers::root))
        // .route("/", routing::post(controllers::insert_url))
        // .route(
        //     "/rpc/shorten-url",
        //     routing::post(controllers::rpc_insert_url),
        // )
        // .route("/leaderboard", routing::get(controllers::leaderboard))
        // .route("/stats/:id", routing::get(controllers::url_stats))
        // .route("/:id", routing::get(controllers::fetch_url))
        .nest_service(
            "/assets",
            ServeDir::new(app_env.config.static_assets_path.as_path()).precompressed_gzip(),
        )
        .with_state(app_env.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], app_env.config.port));

    tracing::info!("Running on port {}", app_env.config.port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(signal_shutdown())
        .await
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    tracing::info!("signal shutdown");
}
