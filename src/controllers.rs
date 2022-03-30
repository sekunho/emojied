use axum::extract::{Extension, Form, Query, Path};
use axum::http::StatusCode;
use hyper::{
    header::{HeaderName, HeaderValue, LOCATION},
    HeaderMap,
};
use axum::response::Json;
use maud::Markup;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use crate::db::{CreateUrl, DbHandle};
use crate::views::{self, root::RootData};
use crate::emoji;

pub async fn root(Query(params): Query<HashMap<String, String>>) -> Markup {
    let custom_url = match params.get(&String::from("custom_url")) {
        Some(_) => true,
        None => false
    };

    views::root::render(RootData { custom_url, identifier: None })
}

pub async fn insert_url(
    db_handle: Extension<Arc<DbHandle>>,
    Form(form_data): Form<CreateUrl>,
    Query(params): Query<HashMap<String, String>>
) -> Markup {
    let custom_url = match params.get(&String::from("custom_url")) {
        Some(_) => true,
        None => false
    };

    match db_handle.insert_url(form_data).await {
        Ok(i) => views::root::render(RootData { custom_url, identifier: Some(i) }),
        Err(_e) => views::root::render(RootData { custom_url, identifier: None }),
    }
}

pub async fn rpc_insert_url(
    db_handle: Extension<Arc<DbHandle>>,
    Json(data): Json<CreateUrl>
) -> (StatusCode, Json<Value>) {
    match db_handle.insert_url(data).await {
        Ok(identifier) => (StatusCode::OK, Json(json!({"identifier": identifier}))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e })))
    }
}

pub async fn fetch_url(
    db_handle: Extension<Arc<DbHandle>>,
    Path(identifier): Path<String>,
) -> (StatusCode, HeaderMap, Markup) {
    let mut headers = HeaderMap::new();

    if emoji::is_emoji(&identifier) {
        match db_handle.fetch_url(identifier).await {
            Ok(u) => {
                headers.insert(LOCATION, u.parse().unwrap());
                (StatusCode::MOVED_PERMANENTLY, headers, maud::html! {})
            },
            Err(_e) => {
                (StatusCode::NOT_FOUND, headers, views::status::not_found())
            },
        }
    } else {
        (StatusCode::BAD_REQUEST, headers, views::status::not_found())
    }
}

pub async fn leaderboard() -> String {
    "hey".to_string()
}

pub async fn js() -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();

    match fs::read_to_string("public/app.js") {
        Ok(content) => {
            headers.insert(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("application/javascript; charset=utf-8"),
            );

            (StatusCode::OK, headers, content)
        }

        Err(_e) => (StatusCode::NOT_FOUND, headers, String::from("Not found")),
    }
}

pub async fn purifyjs() -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();

    match fs::read_to_string("public/purify.min.js") {
        Ok(content) => {
            headers.insert(
                HeaderName::from_static("content-type"),
                HeaderValue::from_static("application/javascript; charset=utf-8"),
            );

            (StatusCode::OK, headers, content)
        }

        Err(_e) => (StatusCode::NOT_FOUND, headers, String::from("Not found")),
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
        views::status::not_found(),
    )
}
