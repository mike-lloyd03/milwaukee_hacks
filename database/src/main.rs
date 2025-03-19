use anyhow::Result;
use sqlx::SqlitePool;
use types::{ProductDB, PromotionDB};

mod config;
mod requests;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::load("config.toml")?;
    let pool = SqlitePool::connect(&config.db_path).await?;

    for promo in config.promos {
        println!("Loading promo {promo}");
        let promo = requests::get_promo(promo)?;
        let promo_db: PromotionDB = promo.into();
        promo_db.create(&pool).await?;

        let products = requests::get_promo_items(promo_db.item_ids.0)?;
        println!("Got {} products", products.len());
        let products_db: Vec<ProductDB> = products.into_iter().map(|p| p.into()).collect();
        for product in products_db {
            product.create(&pool).await?;
        }
    }

    Ok(())
}
