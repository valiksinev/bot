use {
    crate::spot::{f32_deserialize, Side, OrderStatus, TimeInForce},
    hyper::client::connect::Connect,
    serde::Deserialize,
    std::sync::Arc,

};

pub mod base;
mod local_spawner;

pub use base::Futures;
pub use local_spawner::*;

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketOrder {
    client_order_id: String,
    #[serde(deserialize_with = "f32_deserialize")]
    cum_qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    cum_quote: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    executed_qty: f32,
    order_id: u64,
    #[serde(deserialize_with = "f32_deserialize")]
    avg_price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    orig_qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    price: f32,
    reduce_only: bool,
    side: Side,
    position_side: String,
    status: OrderStatus,
    #[serde(deserialize_with = "f32_deserialize")]
    stop_price: f32,
    close_position: bool,
    symbol: String,
    time_in_force: TimeInForce,
    r#type: String,
    orig_type: String,
    update_time: u64,
    working_type: String,
    price_protect: bool,
    price_match: String,
    self_trade_prevention_mode: String,
    good_till_date: u64,
}


