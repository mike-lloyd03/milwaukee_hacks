use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ProductAlgo {
    pub name: String,
    pub price: f32,
}

#[wasm_bindgen]
impl ProductAlgo {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, price: f32) -> Self {
        Self { name, price }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Cart {
    pub items: Vec<ProductAlgo>,
    pub total: f32,
}

#[wasm_bindgen]
impl Cart {
    #[wasm_bindgen(constructor)]
    pub fn new(items: Vec<ProductAlgo>, total: f32) -> Self {
        Self { items, total }
    }
}
