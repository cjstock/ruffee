use sqlx::PgPool;

pub mod coffee_bean;
pub mod grow_event;

pub struct AppState {
    pub db: PgPool,
}
