use axum::{
    Router,
    extract::{OriginalUri, Query},
    routing::get,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .fallback(proxy_handler)
}

async fn health() -> &'static str {
    println!("get: /health");
    "OK"
}

async fn proxy_handler(uri: OriginalUri) -> String {
    println!("full URI: {}", uri.0);
    println!("path: {}", uri.path());

    uri.0.to_string()
}
