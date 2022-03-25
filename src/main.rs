#![forbid(unsafe_code)]

use axum::{
    Router,
    handler::Handler,
    routing::get,
};

use emojiurl::layout;
use emojiurl::fallback;
use emojiurl::assets;

#[tokio::main]
async fn main() {
    let app =
        Router::new()
            .fallback(fallback::not_found.into_service())
            .route("/", get(layout::home))
            .route("/app.css", get(assets::stylesheet));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}


