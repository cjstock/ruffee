use bunnfn::{
    AppState,
    coffee_bean::{get_coffee_by_id, insert_coffee},
    grow_event::NewGrowEvent,
};
use chrono::TimeZone;
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

    // let coffee = insert_coffee(&_app_state.db, "Ethiopia", None).await?;
    //
    // let coffee = get_coffee_by_id(&_app_state.db, &coffee.coffee_bean_id).await?;

    let name = "Coffee with specific harvest date";
    let harvested_date = chrono::Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let new_grow_event = NewGrowEvent {
        timestamp: Some(harvested_date),
        farm_id: None,
        farmer_id: None,
        variety_id: None,
    };
    let coffee = insert_coffee(&_app_state.db, name, Some(new_grow_event)).await?;
    dbg!(&coffee);

    Ok(())
}
