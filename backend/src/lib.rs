use axum::Router;

pub fn router() -> Router {
    Router::new()
        .layer(tower_http::cors::CorsLayer::very_permissive())
}
