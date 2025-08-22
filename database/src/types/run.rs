use anyhow::Result;
use sqlx::SqlitePool;

use crate::now_timestamp;

#[derive(Debug, Default)]
pub struct Run {
    pub id: u32,
    pub total_products: u32,
    pub total_promotions: u32,
    pub start_time: u32,
    pub end_time: u32,
    pub duration: u32,
}

impl Run {
    pub fn start() -> Self {
        Self {
            start_time: now_timestamp(),
            ..Default::default()
        }
    }

    pub fn end(&mut self) {
        self.end_time = now_timestamp();
        self.duration = self.end_time - self.start_time;
    }

    pub async fn write(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            r#"
        insert into runs (
            total_products,
            total_promotions,
            start_time,
            end_time,
            duration
        ) values ($1, $2, $3, $4, $5)
            "#,
            self.total_products,
            self.total_promotions,
            self.start_time,
            self.end_time,
            self.duration
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
