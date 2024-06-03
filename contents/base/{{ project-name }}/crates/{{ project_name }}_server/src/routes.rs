use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> impl IntoResponse {
    "Hello, World!"
}