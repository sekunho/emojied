mod config;
mod controllers;
pub mod db;
mod layouts;
mod components;

use axum::extract::Extension;
use axum::handler::Handler;
use axum::routing;
use axum::Router;
use std::sync::Arc;

pub async fn run(handle: db::DbHandle) -> Result<(), hyper::Error> {
    // https://docs.rs/axum/0.4.8/axum/extract/struct.Extension.html
    // TODO: Read about `Arc` because I have no idea what this does.
    let app_handle = Arc::new(handle);

    let app = Router::new()
        .fallback(controllers::not_found.into_service())
        .route("/", routing::get(controllers::root))
        .route(
            "/",
            routing::post(|handle, body, params| {
                controllers::insert_url(handle, body, params)
            }),
        )
        .route("/rpc/shorten-url", routing::post(controllers::rpc_insert_url))
        .route("/app.css", routing::get(controllers::stylesheet))
        .route("/app.js", routing::get(controllers::js))
        .route("/purify.min.js", routing::get(controllers::purifyjs))
        .layer(Extension(app_handle));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
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
