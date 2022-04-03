use axum::extract::{Extension, Form, Path, Query};
use axum::http::StatusCode;
use axum::response::Json;
use hyper::{
    header::{HeaderName, HeaderValue, LOCATION},
    HeaderMap,
};
use maud::Markup;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use crate::db::{CreateUrl, Handle};
use crate::emoji;
use crate::views::{self, url::RootData};

// TODO: Move stuff to their own modules
// TODO: Implement SSE for URL stats page

pub async fn root(Query(params): Query<HashMap<String, String>>) -> Markup {
    let custom_url = params.get(&String::from("custom_url")).is_some();

    views::url::render(RootData {
        custom_url,
        identifier: None,
    })
}

pub async fn insert_url(
    db_handle: Extension<Arc<Handle>>,
    Form(form_data): Form<CreateUrl>,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, Markup) {
    let custom_url = params.get(&String::from("custom_url")).is_some();

    // TODO: This is so ugly. *spits*
    match db_handle.insert_url(form_data).await {
        Ok(i) => {
            let content = views::url::render(RootData {
                custom_url,
                identifier: Some(i),
            });

            (StatusCode::CREATED, content)
        },
        Err(_e) => {
            let content = views::url::render(RootData {
                custom_url,
                identifier: None,
            });

            (StatusCode::BAD_REQUEST, content)
        },
    }
}

pub async fn rpc_insert_url(
    db_handle: Extension<Arc<Handle>>,
    Json(data): Json<CreateUrl>,
) -> (StatusCode, Json<Value>) {
    match db_handle.insert_url(data).await {
        Ok(identifier) => (StatusCode::CREATED, Json(json!({ "identifier": identifier }))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "message": e }))),
    }
}

pub async fn url_stats(
    db_handle: Extension<Arc<Handle>>,
    Path(identifier): Path<String>,
) -> (StatusCode, Markup) {
    match db_handle.url_stats(identifier).await {
        Ok(url_stat) => (StatusCode::OK, views::url::view_stats(&url_stat)),
        Err(_) => (StatusCode::NOT_FOUND, views::status::not_found()),
    }
}

pub async fn fetch_url(
    db_handle: Extension<Arc<Handle>>,
    Path(identifier): Path<String>,
) -> (StatusCode, HeaderMap, Markup) {
    let mut headers = HeaderMap::new();

    if emoji::is_valid(&identifier) {
        match db_handle.fetch_url(identifier).await {
            Ok(u) => {
                headers.insert(LOCATION, u.parse().unwrap());

                // Not 301 cause 301 gets cached while 307 (temp redirect)
                // isn't cached by the browser.
                (StatusCode::TEMPORARY_REDIRECT, headers, maud::html! {})
            }
            Err(_e) => (StatusCode::NOT_FOUND, headers, views::status::not_found()),
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

pub async fn not_found(_uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        views::status::not_found(),
    )
}
