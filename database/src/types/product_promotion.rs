use anyhow::Result;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct ProductPromotionDB {
    pub id: Option<i64>,
    pub product_id: String,
    pub promotion_id: String,
}

impl ProductPromotionDB {
    pub fn new(product_id: &str, promotion_id: &str) -> Self {
        Self {
            id: None,
            product_id: product_id.to_string(),
            promotion_id: promotion_id.to_string(),
        }
    }

    pub async fn create(&mut self, pool: &SqlitePool) -> Result<()> {
        let id = sqlx::query_scalar!(
            r#"insert into product_promotions (
                product_id,
                promotion_id
            )
            values
            ($1, $2)
            returning id
            "#,
            self.product_id,
            self.promotion_id,
        )
        .fetch_one(pool)
        .await?;

        self.id = Some(id);
        Ok(())
    }
}
