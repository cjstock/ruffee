use axum::Router;

use crate::AppState;

use super::coffee::coffee_router;

pub fn api_router(app_state: AppState) -> Router {
    Router::new().nest("/coffees", coffee_router(app_state))
}
