use std::sync::Arc;

use axum::Router;

pub fn router() -> Router {
    Router::new()
    .route("coffees/", get(get_coffees))
}

async fn get_coffees(ctx: Arc<>)
