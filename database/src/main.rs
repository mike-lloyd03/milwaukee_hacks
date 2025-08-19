use anyhow::Result;
use database::requests::{self, get_product, Brand, Department, SearchParams};
use database::types::{ProductDB, Promotion, PromotionDB};
use database::{config, now_timestamp};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

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
        buy_more_save_more: true,
        buy_one_get_one: true,
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

    for product in &products {
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
    println!("Got {} products", products.len());
    println!("Got {} promos", promos.len());

    for product in products {
        let product_db: ProductDB = product.into();
        product_db.create(&pool).await?;
    }

    for promo in promos {
        // Some promos have items in them which are not fetched by the earlier product
        // search call
        // Might need to ensure these products have the right promo listed under them for back
        // referencing
        for ec in promo.eligibility_criteria.iter() {
            for item_id in ec.item_ids.iter() {
                fetch_product_if_not_exists(&pool, item_id).await?;
            }
        }

        let promo_db: PromotionDB = promo.into();
        promo_db.create(&pool).await?;
    }

    ProductDB::delete_all_before(&pool, now).await?;
    PromotionDB::delete_all_before(&pool, now).await?;

    Ok(())
}

async fn fetch_product_if_not_exists(pool: &SqlitePool, item_id: &str) -> Result<()> {
    let exists = ProductDB::check_exists(pool, item_id).await?;
    if !exists {
        println!("Promo item {item_id} missing from DB. Fetching...");
        let product = get_product(item_id)?;
        let product_db: ProductDB = product.into();
        product_db.create(pool).await?;
    }
    Ok(())
}
