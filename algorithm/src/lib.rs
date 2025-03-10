mod types;
use types::*;

use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bmsm(
    products: JsValue,
    max_cart_size: usize,
    min_total: f32,
    required_products: Vec<String>,
) -> Vec<Cart> {
    let products: Vec<Product> = serde_wasm_bindgen::from_value(products).unwrap();

    let mut combinations: Vec<Vec<&Product>> = vec![];

    for i in 2..max_cart_size {
        combinations.append(&mut products.iter().combinations_with_replacement(i).collect())
    }

    combinations = combinations
        .into_iter()
        .filter(|c| {
            required_products
                .iter()
                .all(|p| c.iter().any(|c| &c.name == p))
        })
        .filter(|c| c.iter().map(|c| c.price).sum::<f32>() >= min_total)
        .collect();

    let mut carts: Vec<Cart> = combinations
        .into_iter()
        .map(|c| {
            let total = c.iter().map(|c| c.price).sum();
            Cart {
                items: c.into_iter().map(|p| p.to_owned()).collect(),
                total,
            }
        })
        .collect();

    carts.sort_by(|a, b| a.total.partial_cmp(&b.total).unwrap());

    carts
}
