use anyhow::Result;
use sqlx::SqlitePool;
use types::PromotionDB;

mod requests;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("data.db").await?;
    let promo = requests::get_promo(320459258)?;
    let promo_db: PromotionDB = promo.into();
    promo_db.create(&pool).await?;

    Ok(())
}
