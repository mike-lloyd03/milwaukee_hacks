use crate::types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bogo(src_products: JsValue, tgt_products: JsValue, required_product: String) -> Cart {
    let src_products: Vec<Product> = serde_wasm_bindgen::from_value(src_products).unwrap();
    let tgt_products: Vec<Product> = serde_wasm_bindgen::from_value(tgt_products).unwrap();

    let items: Vec<Product> = vec![
        get_product_or_required(src_products, &required_product, true),
        get_product_or_required(tgt_products, &required_product, false),
    ]
    .into_iter()
    .filter_map(|p| p)
    .collect();

    let total = items.iter().map(|i| i.price).sum();

    Cart::new(items, total)
}

// Returns the cheapest (or most expensive) product in the vec or the one that matches the `required_product`
fn get_product_or_required(
    products: Vec<Product>,
    required_product: &str,
    cheapest: bool,
) -> Option<Product> {
    if products.is_empty() {
        return None;
    }

    let mut cheapest_product: Option<Product> = None;

    for product in products {
        if product.name == required_product {
            return Some(product);
        }
        match cheapest_product {
            Some(p)
                if (cheapest && product.price < p.price)
                    || (!cheapest && product.price > p.price) =>
            {
                cheapest_product = Some(product)
            }
            None => cheapest_product = Some(product),
            _ => continue,
        }
    }

    cheapest_product
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_get_product_or_required() {
        let products = vec![
            Product {
                name: "product1".to_string(),
                price: 10.0,
            },
            Product {
                name: "product2".to_string(),
                price: 20.0,
            },
            Product {
                name: "product3".to_string(),
                price: 30.0,
            },
        ];

        let required_product = "product7";

        let mut expect = get_product_or_required(products.clone(), required_product, true);

        assert_eq!(expect, Some(products[0].clone()));

        expect = get_product_or_required(products.clone(), required_product, false);

        assert_eq!(expect, Some(products[2].clone()));
    }
}
