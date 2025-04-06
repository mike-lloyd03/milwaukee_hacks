use std::fs;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    pub promos: Vec<Promo>,
    pub db_path: String,
}

#[derive(Deserialize)]
pub struct Promo {
    pub id: u32,
    pub name: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let config_string = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                return Ok(Self::default());
            }
        };

        Ok(toml::from_str(&config_string)?)
    }
}
