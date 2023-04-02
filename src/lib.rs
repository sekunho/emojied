mod components;
pub mod config;
mod controllers;
pub mod db;
mod emoji;
pub mod leaderboard;
pub mod state;
pub mod url;
mod views;

use axum::routing;
use axum::Router;
use state::AppState;
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn run(app_state: AppState) -> Result<(), hyper::Error> {
    let config = app_state.config.clone();
    let handle_state = Arc::new(app_state);

    // https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
    let app = Router::new()
        .fallback(controllers::not_found)
        .route("/", routing::get(controllers::root))
        .route("/", routing::post(controllers::insert_url))
        .route(
            "/rpc/shorten-url",
            routing::post(controllers::rpc_insert_url),
        )
        .route("/leaderboard", routing::get(controllers::leaderboard))
        .route("/app.css", routing::get(controllers::stylesheet))
        .route("/app.js", routing::get(controllers::js))
        .route("/purify.min.js", routing::get(controllers::purifyjs))
        .route("/stats/:id", routing::get(controllers::url_stats))
        .route("/:id", routing::get(controllers::fetch_url))
        .with_state(handle_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    println!("Running on port {}", config.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
