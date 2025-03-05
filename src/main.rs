use std::fs;

mod types;
use types::*;

use itertools::Itertools;

// The most amount of items in a cart
const MAX_CART_SIZE: usize = 5;
// The minimum total cost of the cart
const MIN_CART_TOTAL: f32 = 1000.0;
// List of products that must be in the cart (should be less than MAX_CART_SIZE)
const REQUIRED_PRODUCTS: &[&str] = &["Track Saw", "Jig Saw"];

fn main() {
    if !(2..=7).contains(&MAX_CART_SIZE) {
        eprintln!("cart_size must be 2 < x <= 7");
        return;
    }

    let products = load_products("products.toml");

    let carts = get_cart_options(&products, MAX_CART_SIZE, MIN_CART_TOTAL, REQUIRED_PRODUCTS);

    println!(
        "Cheapest carts with {:?} over ${}:\n",
        REQUIRED_PRODUCTS, MIN_CART_TOTAL
    );
    for (i, cart) in carts.iter().take(3).enumerate() {
        println! {"Option {}:", i+1}
        println!("{cart}");
        println!("------------\n")
    }
}

fn get_cart_options<'a>(
    products: &'a [Product],
    max_cart_size: usize,
    min_total: f32,
    required_products: &'a [&str],
) -> Vec<Cart<'a>> {
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
            Cart { items: c, total }
        })
        .collect();

    carts.sort_by(|a, b| a.total.partial_cmp(&b.total).unwrap());

    carts
}

fn load_products(filename: &str) -> Vec<Product> {
    let toml_string = fs::read_to_string(filename).expect("Failed to read file");
    let products_file: ProductsFile = toml::from_str(&toml_string).expect("Failed to parse TOML");
    products_file.products
}
