use {
    serde::Deserialize,
    crate::spot::f32_deserialize,
};

pub mod base;
pub use base::Futures;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTicker {
    pub last_update_id: u64,
    pub symbol: String,
    #[serde(deserialize_with = "f32_deserialize")]
    pub bid_price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    pub bid_qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    pub ask_price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    pub ask_qty: f32,
    pub time: u64,
}


