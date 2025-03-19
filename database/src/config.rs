use std::fs;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    pub promos: Vec<u32>,
    pub db_path: String,
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
