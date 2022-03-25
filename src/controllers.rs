use axum::extract::{Extension, Form};
use axum::http::StatusCode;
use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};
use maud::Markup;
use std::fs;
use std::sync::Arc;

use crate::db::{CreateUrl, DbHandle};
use crate::layouts;

pub async fn root() -> Markup {
    layouts::home()
}

pub async fn insert_url(
    db_handle: Extension<Arc<DbHandle>>,
    Form(form_data): Form<CreateUrl>,
) -> (StatusCode, HeaderMap, String) {
    match db_handle.insert_url(form_data).await {
        Ok(_) => (StatusCode::OK, HeaderMap::new(), String::from("OK")),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            HeaderMap::new(),
            String::from("Internal server error"),
        ),
    }
}

pub async fn stylesheet() -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();

    match fs::read_to_string("public/app.css") {
        Ok(content) => {
            headers.insert(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("text/css; charset=utf-8"),
            );

            (StatusCode::OK, headers, content)
        }

        Err(_e) => (StatusCode::NOT_FOUND, headers, String::from("Not found")),
    }
}

pub async fn not_found(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}
