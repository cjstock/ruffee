use anyhow::Context;
use axum::Router;
use ruffee::app::router::app_router;
use ruffee::{api::router::api_router, AppState};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ruffee=debug,tower_http=debug,axum::rejection=trace".into()),
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

    let router = Router::new()
        .nest("/api", api_router(app_state.clone()))
        .nest("/app", app_router(app_state.clone()))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(TraceLayer::new_for_http());
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let tcplistener = TcpListener::bind(addr).await?;

    info!("router initialized, now listening on port {}", port);

    axum::serve(tcplistener, router.into_make_service())
        .await
        .context("error starting server")?;

    Ok(())
}
