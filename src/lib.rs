use sqlx::PgPool;

pub mod coffee_bean;

pub struct AppState {
    pub db: PgPool,
}
