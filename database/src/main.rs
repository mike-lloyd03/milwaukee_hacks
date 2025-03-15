use anyhow::Result;

mod requests;
mod types;

fn main() -> Result<()> {
    let promo = requests::get_promo(320459258)?;
    let item_ids = promo.eligibility_criteria[0].item_ids.clone();
    let products = requests::get_promo_items(item_ids)?;

    for product in products {
        println!("name: {}", product.identifiers.product_label);
        println!("price: {}", product.pricing.value);
        println!("==================")
    }

    Ok(())
}
