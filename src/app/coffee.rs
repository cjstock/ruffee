use askama::Template;
use axum::{
    debug_handler,
    extract::State,
    routing::{get, post},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, Result};

pub(crate) fn coffee_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_coffees))
        .route("/add", get(page_add_coffee).post(add_coffee))
        .with_state(app_state)
}

#[derive(Serialize, sqlx::FromRow, Deserialize)]
struct Coffee {
    coffee_id: Uuid,
    title: String,
    altitude: Option<i32>,
    description: Option<String>,
    country: Option<String>,
    region: Option<String>,
    farm: Option<String>,
    farmer: Option<String>,
    variety: Option<String>,
    process: Option<String>,
    grade: Option<String>,
    roast_level: Option<String>,
    tasting_notes: Option<Vec<String>>,
    recommended_brew_methods: Option<Vec<String>>,
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

#[derive(Template)]
#[template(path = "coffee.html")]
struct CoffeeCardTemplate {
    coffee: Coffee,
}

#[debug_handler]
async fn add_coffee(
    State(app_state): State<AppState>,
    Form(coffee): Form<Coffee>,
) -> Result<CoffeeCardTemplate> {
    let result = sqlx::query!(r#"
        insert into coffee
        (title, altitude, description, country, region, farm, farmer, variety, process, grade, roast_level, tasting_notes, recommended_brew_methods) values
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        returning coffee_id
    "#, coffee.title, coffee.altitude, coffee.description, coffee.country, coffee.region, coffee.farm, coffee.farmer, coffee.variety, coffee.process, coffee.grade, coffee.roast_level, coffee.tasting_notes.as_deref(), coffee.recommended_brew_methods.as_deref()).fetch_one(&app_state.db).await?;
    tracing::trace!("coffee added with id: {:?}", result);
    Ok(CoffeeCardTemplate { coffee })
}

#[derive(Template)]
#[template(path = "add_coffee.html")]
struct PageAddCoffee {}

async fn page_add_coffee() -> Result<PageAddCoffee> {
    Ok(PageAddCoffee {})
}
