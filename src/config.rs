use {
    serde::Deserialize,
    std::{
        fs::File, io, io::Read,
    },
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub spot_url: String,
    pub usd_futures_url: String,
    pub symbol: String,
    pub min_order_size: u64,
    pub max_order_size: u64,
    pub total_input_amount: u64,
    pub spot_api_key: String,
    pub spot_api_secret: String,
    pub futures_api_key: String,
    pub futures_api_secret: String,
}

impl Config {
    pub fn read(path: &str) -> io::Result<Config> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        Ok(serde_json::from_str(&s)?)
    }
}



