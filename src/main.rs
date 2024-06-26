use std::sync::Arc;

use anyhow::Context;
use askama_axum::Template;
use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, query, query_as, PgPool};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ruffee".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_conn = dotenv::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let app_state = AppState { db: pool };

    info!("initializing router");
    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new().route("/coffee", get(get_coffees));

    let router = Router::new()
        .nest("/api", api_router)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(app_state);
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let tcplistener = TcpListener::bind(addr).await?;

    info!("router initialized, now listening on port {}", port);

    axum::serve(tcplistener, router.into_make_service())
        .await
        .context("error starting server")?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Deserialize, Serialize)]
struct Coffee {
    title: String,
    description: String,
    altitude: i64,
}

#[derive(Deserialize, Serialize)]
struct MultipleCoffee {
    coffees: Vec<Coffee>,
    coffees_count: usize,
}

async fn get_coffees(State(state): State<AppState>) -> anyhow::Result<Json<Vec<Coffee>>> {
    let coffees = query!("select * from coffee").fetch_all(state.db).await?;
    Ok()
}

#[derive(Template)]
#[template(path = "coffees.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}

#[derive(Template)]
#[template(path = "another-page.html")]
struct AnotherPageTemplate<'a> {
    name: &'a str,
}

async fn another_page() -> AnotherPageTemplate<'static> {
    AnotherPageTemplate {
        name: "another one",
    }
}
