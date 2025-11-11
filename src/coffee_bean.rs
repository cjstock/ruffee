use sqlx::types::Uuid;
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Debug)]
pub struct CoffeeBean {
    pub coffee_bean_id: Uuid,
    pub name: String,
    pub grow_event_id: Option<Uuid>,
    pub mill_event_id: Option<Uuid>,
    pub roast_event_id: Option<Uuid>,
}
pub async fn insert_coffee(pool: PgPool, name: &str) -> color_eyre::Result<CoffeeBean> {
    let coffee: CoffeeBean = sqlx::query_as!(
        CoffeeBean,
        "insert into coffee_bean (name) values ($1) returning *",
        name
    )
    .fetch_one(&pool)
    .await?;

    Ok(coffee)
}

#[cfg(test)]
mod test {
    use sqlx::PgPool;

    use crate::coffee_bean::CoffeeBean;

    #[sqlx::test]
    async fn get_coffees_test(pool: PgPool) -> sqlx::Result<()> {
        let foo = sqlx::query_as!(CoffeeBean, "select * from coffee_bean")
            .fetch_all(&pool)
            .await?;

        Ok(())
    }
    #[sqlx::test]
    async fn insert_coffee_test(pool: PgPool) -> color_eyre::Result<()> {
        let coffee: CoffeeBean = sqlx::query_as!(
            CoffeeBean,
            "insert into coffee_bean (name) values ($1) returning *",
            "Ethiopia".to_string()
        )
        .fetch_one(&pool)
        .await?;

        dbg!(&coffee);
        Ok(())
    }
}
