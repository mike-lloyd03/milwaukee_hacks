use std::fs;

mod types;
use types::*;

use itertools::Itertools;

fn main() {
    let cart_size = 4;
    let min_amount = 1000.0;

    let toml_string = fs::read_to_string("products.toml").expect("Failed to read file");
    let products_file: ProductsFile = toml::from_str(&toml_string).expect("Failed to parse TOML");

    let required_products = ["Track Saw"].map(|s| s.to_string());

    let combinations: Vec<Vec<Product>> = products_file
        .products
        .into_iter()
        .combinations_with_replacement(cart_size)
        .filter(|c| c.iter().any(|p| required_products.contains(&p.name)))
        .filter(|c| c.iter().map(|c| c.price).sum::<f32>() > min_amount)
        .collect();

    let mut carts: Vec<Cart> = combinations
        .into_iter()
        .map(|c| {
            let total = c.iter().map(|c| c.price).sum();
            Cart { items: c, total }
        })
        .collect();

    carts.sort_by(|a, b| a.total.partial_cmp(&b.total).unwrap());

    println!(
        "Cheapest carts with {:?} over ${}:\n",
        required_products, min_amount
    );
    for (i, cart) in carts.iter().take(3).enumerate() {
        println! {"Option {}:", i+1}
        println!("{cart}");
        println!("------------\n")
    }
}
