use axum::routing::{get, post};

pub fn route() -> axum::Router {
    let router = axum::Router::new();
    router
        .route("/", get(|| async { "Hello, World!" }))
        .route("/tasks/:id", get(controllers::todo::retrieve::handle))
        .route("/tasks", post(controllers::todo::post::handle))
}
