use anyhow::Context;
use askama_axum::Template;
use axum::{routing::get, Router};
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

    info!("initializing router");
    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new().route("/hello", get(say_hello));

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(hello))
        .route("/another-page", get(another_page))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let tcplistener = TcpListener::bind(addr).await?;

    info!("router initialized, now listening on port {}", port);

    axum::serve(tcplistener, router.into_make_service())
        .await
        .context("error starting server")?;

    Ok(())
}

async fn say_hello() -> &'static str {
    "Yoooooo"
}

#[derive(Template)]
#[template(path = "hello.html")]
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
