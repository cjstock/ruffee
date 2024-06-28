use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

use crate::AppState;
use crate::Result;

#[derive(Serialize, sqlx::FromRow)]
struct Coffee {
    title: String,
    description: Option<String>,
    altitude: Option<i64>,
    country: Option<String>,
    region: Option<String>,
    farm: Option<String>,
    farmer: Option<String>,
    variety: Option<String>,
    process: Option<String>,
    grade: Option<String>,
    roast_level: Option<String>,
    tasting_notes: Vec<String>,
    recommended_brew_methods: Option<Vec<String>>,
}

#[derive(Serialize)]
struct MultipleCoffeesBody {
    coffees: Vec<Coffee>,
}

pub(crate) fn coffee_router(state: AppState) -> Router {
    Router::new().route("/", get(get_coffees)).with_state(state)
}

async fn get_coffees(State(app_state): State<AppState>) -> Result<Json<MultipleCoffeesBody>> {
    let coffees = sqlx::query_as::<_, Coffee>("select * from coffee")
        .fetch_all(&app_state.db)
        .await?;
    Ok(Json(MultipleCoffeesBody { coffees }))
}
