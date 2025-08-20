use std::collections::HashSet;

use anyhow::Result;
use database::requests::{self, get_product, Brand, Department, SearchParams};
use database::types::{ProductDB, Promotion, PromotionDB};
use database::{config, now_timestamp};
use futures::future::join_all;
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
            match requests::get_promos_for_product(&product.id) {
                Ok(p) => {
                    for promo in p {
                        println!(
                            "Got promo {}: {}",
                            &promo.promotion_id,
                            promo
                                .description
                                .long_desc
                                .clone()
                                .unwrap_or(promo.description.short_desc.clone())
                        );

                        promos.push(promo);
                    }
                }
                Err(e) => {
                    let err_msg = e.to_string();
                    let skip_errors = ["Promotion not found", "No promotions for product"];

                    if skip_errors.iter().any(|e| err_msg.contains(e)) {
                        println!("Warning: {e}");
                        continue;
                    }
                    return Err(e);
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

    let mut items_to_fetch: HashSet<String> = HashSet::new();

    for promo in promos {
        // Some promos have items in them which are not fetched by the earlier product
        // search call
        // Might need to ensure these products have the right promo listed under them for back
        // referencing
        for ec in promo.eligibility_criteria.iter() {
            for item_id in ec.item_ids.iter() {
                if !ProductDB::check_exists(&pool, item_id).await? {
                    items_to_fetch.insert(item_id.to_string());
                };
            }
        }

        let promo_db: PromotionDB = promo.into();
        promo_db.create(&pool).await?;
    }

    let futures = items_to_fetch.iter().map(|item_id| {
        let id_clone = item_id.clone();

        tokio::task::spawn_blocking(move || {
            println!("Promo item {id_clone} missing from DB. Fetching...");
            let product = get_product(&id_clone).unwrap();
            let product_db: ProductDB = product.into();
            product_db
        })
    });

    let results = join_all(futures).await;

    let new_products: Vec<ProductDB> = results.into_iter().filter_map(Result::ok).collect();

    let mut tx = pool.begin().await?;

    println!("Adding {} products...", new_products.len());
    for product in new_products {
        product.create(&mut *tx).await?;
    }

    tx.commit().await?;

    ProductDB::delete_all_before(&pool, now).await?;
    PromotionDB::delete_all_before(&pool, now).await?;

    Ok(())
}
