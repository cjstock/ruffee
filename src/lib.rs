pub mod api;
pub mod app;
mod error;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub use error::Error;
use sqlx::PgPool;

pub type Result<T, E = Error> = std::result::Result<T, E>;
