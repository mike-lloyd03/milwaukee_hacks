use anyhow::Result;
use database::now_timestamp;
use requests::{Brand, Department, SearchParams};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use types::{ProductDB, ProductPromotionDB, Promotion, PromotionDB};

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
        department: Some(Department::PowerTools),
        brand: Some(Brand::Milwaukee),
        buy_more_save_more: false,
        buy_one_get_one: false,
        special_buy: false,
    };

    println!(
        "Searching for products with brand {:?}, bmsm: {}, bogo: {}, special buy: {}. nav_param: {}",
        search_params.brand,
        search_params.buy_more_save_more,
        search_params.buy_one_get_one,
        search_params.special_buy,
        search_params.to_nav_param()
    );

    let now = now_timestamp();

    let mut promos: Vec<Promotion> = Vec::new();
    let products = requests::get_products(search_params)?;
    let mut product_promos_db: Vec<ProductPromotionDB> = Vec::new();

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

        for product_promo in &product_promos {
            let p = ProductPromotionDB::new(&product.id, product_promo);
            product_promos_db.push(p);
        }
    }
    println!("Got {} products", products.len());
    println!("Got {} promos", promos.len());

    for product in products {
        let product_db: ProductDB = product.into();
        product_db.create(&pool).await?;
    }

    for promo in promos {
        let promo_db: PromotionDB = promo.into();
        promo_db.create(&pool).await?;
    }

    for mut product_promo in product_promos_db {
        product_promo.create(&pool).await?;
    }

    ProductDB::delete_all_before(&pool, now).await?;
    PromotionDB::delete_all_before(&pool, now).await?;

    Ok(())
}
