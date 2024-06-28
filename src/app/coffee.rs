use askama::Template;
use axum::{extract::State, routing::get, Router};
use serde::Serialize;

use crate::{AppState, Result};

pub(crate) fn coffee_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_coffees))
        .with_state(app_state)
}

#[derive(Serialize, sqlx::FromRow)]
struct Coffee {
    title: String,
    altitude: Option<i64>,
    description: Option<String>,
}

#[derive(Template)]
#[template(path = "coffees.html")]
struct CoffeesTemplate {
    coffees: Vec<Coffee>,
}

async fn get_coffees(State(app_state): State<AppState>) -> Result<CoffeesTemplate> {
    let coffees = sqlx::query_as::<_, Coffee>("select * from coffee")
        .fetch_all(&app_state.db)
        .await?;
    Ok(CoffeesTemplate { coffees })
}
