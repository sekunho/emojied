mod components;
pub mod config;
mod controllers;
pub mod db;
mod emoji;
pub mod url;
mod views;

use axum::extract::Extension;
use axum::handler::Handler;
use axum::routing;
use axum::Router;
use std::sync::Arc;

pub async fn run(handle: db::Handle) -> Result<(), hyper::Error> {
    // TODO: Read about `Arc` because I have no idea what this does.
    let handle = Arc::new(handle);

    // https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
    let app = Router::new()
        .fallback(controllers::not_found.into_service())
        .route("/", routing::get(controllers::root))
        .route("/", routing::post(controllers::insert_url))
        .route(
            "/rpc/shorten-url",
            routing::post(controllers::rpc_insert_url),
        )
        .route("/app.css", routing::get(controllers::stylesheet))
        .route("/app.js", routing::get(controllers::js))
        .route("/purify.min.js", routing::get(controllers::purifyjs))
        .route("/stats/:id", routing::get(controllers::url_stats))
        .route("/:id", routing::get(controllers::fetch_url))
        .layer(Extension(handle));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
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
