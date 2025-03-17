use anyhow::Result;
use sqlx::SqlitePool;
use types::{ProductDB, PromotionDB};

mod requests;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("data.db").await?;

    let promo = requests::get_promo(320459258)?;
    let promo_db: PromotionDB = promo.into();
    promo_db.create(&pool).await?;

    let products = requests::get_promo_items(promo_db.item_ids.0)?;
    let products_db: Vec<ProductDB> = products.into_iter().map(|p| p.into()).collect();
    for product in products_db {
        product.create(&pool).await?;
    }

    Ok(())
}
