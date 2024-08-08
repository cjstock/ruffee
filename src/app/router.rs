use super::coffee::coffee_router;
use crate::AppState;
use axum::Router;

pub fn app_router(app_state: AppState) -> Router {
    Router::new().nest("/coffee", coffee_router(app_state))
}
