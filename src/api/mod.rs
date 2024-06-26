use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;

mod coffee;

pub struct AppContext {
    db: PgPool,
}
