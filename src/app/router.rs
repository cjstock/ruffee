use super::coffee::coffee_router;
use crate::AppState;
use axum::Router;

pub fn app_router(app_state: AppState) -> Router {
    Router::new().nest("/coffees", coffee_router(app_state))
}
