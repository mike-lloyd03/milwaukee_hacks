use crate::types::*;
use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bmsm(
    products: JsValue,
    min_cart_size: usize,
    max_cart_size: usize,
    min_total: f32,
    required_products: Vec<String>,
) -> Vec<Cart> {
    let products: Vec<ProductAlgo> = serde_wasm_bindgen::from_value(products).unwrap();

    let mut combinations: Vec<Vec<&ProductAlgo>> = vec![];

    for i in min_cart_size..=max_cart_size {
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

    carts.sort_by(|a, b| a.total.partial_cmp(&b.total).expect("carts should sort"));

    carts
}
