use std::sync::Arc;

use super::client::Forwarder;
use axum::{
    Router,
    body::Body,
    extract::{OriginalUri, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

#[derive(Clone)]
struct AppState {
    forwarder: Arc<Forwarder>,
    origin: Arc<String>,
}

pub fn create_router(origin: String) -> Router {
    let state = AppState {
        forwarder: Arc::new(Forwarder::new()),
        origin: Arc::new(origin),
    };

    Router::new()
        .route("/health", get(health))
        .fallback(proxy_handler)
        .with_state(state)
}

async fn health() -> &'static str {
    println!("get: /health");
    "OK"
}

async fn proxy_handler(State(state): State<AppState>, uri: OriginalUri) -> Response {
    let path_and_query = uri.0.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");

    match state
        .forwarder
        .forward(&state.origin, reqwest::Method::GET, path_and_query, None)
        .await
    {
        Ok(resp) => {
            let status =
                StatusCode::from_u16(resp.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            let mut builder = Response::builder().status(status);

            if let Some(headers) = builder.headers_mut() {
                *headers = resp.headers;
            }

            builder.body(Body::from(resp.body)).unwrap_or_else(|_| {
                (StatusCode::INTERNAL_SERVER_ERROR, "response build failed").into_response()
            })
        }
        Err(e) => {
            eprintln!("forward error: {e}");
            (StatusCode::BAD_GATEWAY, format!("Bad Gateway: {e}")).into_response()
        }
    }
}
