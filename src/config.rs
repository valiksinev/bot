use std::io;
use {
    serde::Deserialize,
    std::{
        fs::File, io::Read,
    },
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub spot_url: String,
    pub usd_futures_url: String,
    pub symbol: String,
    pub min_order_size: u32,
    pub max_order_size: u32,
    pub total_input_amount: u32,
}


impl Config {
    pub fn read(path: &str) -> std::io::Result<Config> {
        let mut f = File::open(path)?;
            // .with_context(|| format!("Failed to read config.json from {}", path))?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;


        Ok(serde_json::from_str(&s)?)
    }
}



