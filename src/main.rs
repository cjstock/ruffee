use bunnfn::{AppState, coffee_bean};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let db_conn = dotenv::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let _app_state = AppState { db: pool };

    let coffee = coffee_bean::insert_coffee(_app_state.db, "Ethiopia").await?;

    dbg!(&coffee);

    Ok(())
}
