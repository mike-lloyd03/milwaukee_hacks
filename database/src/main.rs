use std::collections::{HashMap, HashSet};

use anyhow::Result;
use database::requests::{self, get_product, Brand, Department, SearchParams};
use database::types::{Product, ProductDB, Promotion, PromotionDB};
use database::{config, now_timestamp};
use futures::future::join_all;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

#[tokio::main]
async fn main() -> Result<()> {
    let load = config::Config::load("config.toml")?;
    let config = load;
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

    let products = requests::get_products(search_params)?;

    let promos = get_all_promos(&products).await;

    let additional_products = get_additional_products(&products, &promos).await?;

    println!("Got {} products", products.len());
    println!("Got {} promos", promos.len());
    println!("Got {} additional products", additional_products.len());

    let mut tx = pool.begin().await?;

    for product in products {
        let product_db: ProductDB = product.into();
        product_db.create(&mut *tx).await?;
    }

    for promo in promos {
        let promo_db: PromotionDB = promo.into();
        promo_db.create(&mut *tx).await?;
    }

    for product in additional_products {
        let product_db: ProductDB = product.into();
        product_db.create(&mut *tx).await?;
    }

    tx.commit().await?;

    ProductDB::delete_all_before(&pool, now).await?;
    PromotionDB::delete_all_before(&pool, now).await?;

    Ok(())
}

async fn get_all_promos(products: &Vec<Product>) -> Vec<Promotion> {
    // Map of promo_id: product item_id that returns that promo
    let mut expected_promo_ids: HashMap<u32, String> = HashMap::new();

    for product in products {
        product
            .pricing
            .conditional_promotions
            .iter()
            .for_each(|promo| {
                if let Some(promo_id) = promo.promotion_id {
                    expected_promo_ids
                        .entry(promo_id)
                        .or_insert(product.id.clone());
                }
            });
    }

    println!("Expecting {} promos", expected_promo_ids.len());

    let futures = expected_promo_ids.values().map(|item_id| {
        let id_clone = item_id.clone();

        tokio::task::spawn_blocking(move || {
            let promos = requests::get_promos_for_product(&id_clone).unwrap();
            promos.iter().for_each(|promo| {
                println!(
                    "Got promo {}: {}",
                    &promo.promotion_id,
                    promo
                        .description
                        .long_desc
                        .clone()
                        .unwrap_or(promo.description.short_desc.clone())
                );
            });
            promos
        })
    });

    let results = join_all(futures).await;

    results
        .into_iter()
        .filter_map(Result::ok)
        .flatten()
        .collect()
}

async fn get_additional_products(
    products: &[Product],
    promos: &[Promotion],
) -> Result<Vec<Product>> {
    let mut items_to_fetch: HashSet<String> = HashSet::new();

    for promo in promos {
        // Some promos have items in them which are not fetched by the earlier product
        // search call
        // Might need to ensure these products have the right promo listed under them for back
        // referencing
        for ec in promo.eligibility_criteria.iter() {
            for item_id in ec.item_ids.iter() {
                if !products.iter().any(|p| p.id == *item_id) {
                    items_to_fetch.insert(item_id.to_string());
                }
            }
        }
    }

    let futures = items_to_fetch.iter().map(|item_id| {
        let id_clone = item_id.clone();

        tokio::task::spawn_blocking(move || {
            println!("Promo item {id_clone} missing from DB. Fetching...");
            get_product(&id_clone).unwrap()
        })
    });

    let results = join_all(futures).await;

    Ok(results.into_iter().filter_map(Result::ok).collect())
}
