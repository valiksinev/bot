use serde::{
    Deserialize,
};

pub mod base;

pub use base::Futures;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTiker {
    last_update_id: u64,
    symbol: String,
    bid_price: String,
    bid_qty: String,
    ask_price: String,
    ask_qty: String,
    time: u64,
}


