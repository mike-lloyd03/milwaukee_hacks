use anyhow::Result;
use database::now_timestamp;
use requests::{Brand, SearchParams};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use types::{ProductDB, Promotion, PromotionDB};

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

    let search_params = SearchParams {
        brand: Some(Brand::Milwaukee),
        buy_more_save_more: true,
        buy_one_get_one: true,
        special_buy: false,
    };

    let now = now_timestamp();

    let mut promos: Vec<Promotion> = Vec::new();
    let products = requests::get_products(search_params)?;

    for product in &products {
        println!("Got product {}", product.id);
        let product_promos: Vec<String> = product
            .pricing
            .conditional_promotions
            .iter()
            .filter_map(|p| p.promotion_id.map(|v| v.to_string()))
            .collect();

        let existing_promo_ids: Vec<String> =
            promos.iter().map(|p| p.promotion_id.clone()).collect();

        if !existing_promo_ids
            .iter()
            .any(|existing_promo| product_promos.contains(existing_promo))
        {
            match requests::get_promo(&product.id) {
                Ok(p) => {
                    println!(
                        "Got promo {}: {}",
                        &p.promotion_id,
                        p.description
                            .long_desc
                            .clone()
                            .unwrap_or(p.description.short_desc.clone())
                    );
                    promos.push(p);
                }
                Err(e) => {
                    println!("Warning: {e}");
                    continue;
                }
            };
        }
    }

    let products_db: Vec<ProductDB> = products.into_iter().map(|p| p.into()).collect();
    for product in &products_db {
        product.create(&pool).await?;
    }

    let promos_db: Vec<PromotionDB> = promos.into_iter().map(|p| p.into()).collect();
    for promo in &promos_db {
        promo.create(&pool).await?;
    }

    println!("Got {} products", products_db.len());
    println!("Got {} promos", promos_db.len());

    ProductDB::delete_all_before(&pool, now).await?;
    PromotionDB::delete_all_before(&pool, now).await?;

    Ok(())
}
