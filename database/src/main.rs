use anyhow::Result;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use types::{ProductDB, PromotionDB};

mod config;
mod requests;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::load("config.toml")?;
    let options = SqliteConnectOptions::new()
        .filename(&config.db_path)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    for promo in config.promos {
        println!("Loading promo {}", promo.name);
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
