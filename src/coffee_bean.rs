use crate::grow_event::{GrowEvent, NewGrowEvent};
use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct CoffeeBean {
    pub coffee_bean_id: Uuid,
    pub name: String,
    pub grow_event_id: Option<Uuid>,
    pub mill_event_id: Option<Uuid>,
    pub roast_event_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
pub async fn insert_coffee(
    pool: &PgPool,
    name: &str,
    grow_event: Option<NewGrowEvent>,
) -> color_eyre::Result<CoffeeBean> {
    let mut tx = pool.begin().await?;

    let grow_event_id = if let Some(new_grow_event) = grow_event {
        let inserted_grow_event = sqlx::query_as!(
            GrowEvent,
            r#"
            insert into grow_event (timestamp, farm_id, farmer_id, variety_id)
            values ($1, $2, $3, $4)
            returning *
            "#,
            new_grow_event.timestamp,
            new_grow_event.farm_id,
            new_grow_event.farmer_id,
            new_grow_event.variety_id
        )
        .fetch_one(&mut *tx)
        .await?;
        Some(inserted_grow_event.grow_event_id)
    } else {
        None
    };

    let coffee: CoffeeBean = sqlx::query_as!(
        CoffeeBean,
        "insert into coffee_bean (name, grow_event_id) values ($1, $2) returning *",
        name,
        grow_event_id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(coffee)
}

pub async fn get_coffee_by_id(pool: &PgPool, id: &Uuid) -> color_eyre::Result<CoffeeBean> {
    let coffee: CoffeeBean = sqlx::query_as!(
        CoffeeBean,
        "select * from coffee_bean where coffee_bean_id = ($1)",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(coffee)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;
    use sqlx::PgPool;

    #[sqlx::test]
    fn insert_ethiopia(pool: PgPool) -> color_eyre::Result<()> {
        let name = "Ethiopia";
        let _ = insert_coffee(&pool, name, None).await?;
        Ok(())
    }

    #[sqlx::test]
    fn get_ethiopia(pool: PgPool) -> color_eyre::Result<()> {
        let name = "Ethiopia";
        let coffee = insert_coffee(&pool, name, None).await?;
        let res_name = get_coffee_by_id(&pool, &coffee.coffee_bean_id).await?.name;

        assert_eq!(name, res_name);

        Ok(())
    }

    #[sqlx::test]
    fn insert_with_grow_event(pool: PgPool) -> color_eyre::Result<()> {
        let name = "Ethiopia with Grow Event";
        let new_grow_event = crate::grow_event::NewGrowEvent {
            timestamp: Some(chrono::Utc::now()),
            farm_id: None,
            farmer_id: None,
            variety_id: None,
        };
        let coffee = insert_coffee(&pool, name, Some(new_grow_event)).await?;
        assert_eq!(name, coffee.name);
        assert!(coffee.grow_event_id.is_some());
        Ok(())
    }

    #[sqlx::test]
    fn insert_with_specific_grow_event_date(pool: PgPool) -> color_eyre::Result<()> {
        let name = "Coffee with specific harvest date";
        let harvested_date = chrono::Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let new_grow_event = crate::grow_event::NewGrowEvent {
            timestamp: Some(harvested_date),
            farm_id: None,
            farmer_id: None,
            variety_id: None,
        };
        let coffee = insert_coffee(&pool, name, Some(new_grow_event)).await?;
        assert_eq!(name, coffee.name);
        assert!(coffee.grow_event_id.is_some());

        // Verify the timestamp date
        let grow_event_id = coffee.grow_event_id.unwrap();
        let grow_event: crate::grow_event::GrowEvent = sqlx::query_as!(
            crate::grow_event::GrowEvent,
            "select * from grow_event where grow_event_id = $1",
            grow_event_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(
            grow_event.timestamp.unwrap().date_naive(),
            harvested_date.date_naive()
        );

        Ok(())
    }

    #[sqlx::test]
    fn test_insert_grow_event_directly(pool: PgPool) -> color_eyre::Result<()> {
        let new_grow_event = crate::grow_event::NewGrowEvent {
            timestamp: Some(chrono::Utc::now()),
            farm_id: None,
            farmer_id: None,
            variety_id: None,
        };

        let inserted_grow_event = sqlx::query_as!(
            crate::grow_event::GrowEvent,
            r#"
            insert into grow_event (timestamp, farm_id, farmer_id, variety_id)
            values ($1, $2, $3, $4)
            returning *
            "#,
            new_grow_event.timestamp,
            new_grow_event.farm_id,
            new_grow_event.farmer_id,
            new_grow_event.variety_id
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(
            inserted_grow_event.timestamp.unwrap().date_naive(),
            new_grow_event.timestamp.unwrap().date_naive()
        );
        Ok(())
    }
}
