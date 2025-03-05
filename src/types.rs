use std::fmt::Display;

use serde::Deserialize;
#[derive(Debug, PartialEq, PartialOrd, Deserialize)]
pub struct Product {
    pub name: String,
    pub price: f32,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ${}", self.name, self.price)
    }
}

pub struct Cart<'a> {
    pub items: Vec<&'a Product>,
    pub total: f32,
}

impl Display for Cart<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        write!(f, "Total: ${}", self.total)
    }
}

#[derive(Debug, Deserialize)]
pub struct ProductsFile {
    pub products: Vec<Product>,
}
