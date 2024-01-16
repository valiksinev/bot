use serde::{
    de, Deserialize, Deserializer,
};

pub mod base;
pub use base::Spot;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    pub symbol: String,
    #[serde(deserialize_with = "f32_deserialize")]
    pub bid_price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    pub bid_qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    pub ask_price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    pub ask_qty: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
    ExpiredInMatch,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Fill {
    #[serde(deserialize_with = "f32_deserialize")]
    price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    commission: f32,
    commission_asset: String,
    trade_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LimitOrder {
    symbol: String,
    order_id: u64,
    order_list_id: i64,
    client_order_id: String,
    transact_time: u64,
    #[serde(deserialize_with = "f32_deserialize")]
    price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    orig_qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    executed_qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    cummulative_quote_qty: f32,
    status: OrderStatus,
    time_in_force: TimeInForce,
    r#type: OrderType,
    side: Side,
    working_time: u64,
    fills : Vec<Fill>,
    self_trade_prevention_mode: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QueryOrder {
    symbol: String,
    order_id: u64,
    order_list_id: i64,
    client_order_id: String,
    #[serde(deserialize_with = "f32_deserialize")]
    price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    orig_qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    executed_qty: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    cummulative_quote_qty: f32,
    status: OrderStatus,
    time_in_force: TimeInForce,
    r#type: OrderType,
    side: Side,
    #[serde(deserialize_with = "f32_deserialize")]
    stop_price: f32,
    #[serde(deserialize_with = "f32_deserialize")]
    iceberg_qty: f32,
    time: u64,
    update_time: u64,
    is_working: bool,
    working_time: u64,
    #[serde(deserialize_with = "f32_deserialize")]
    orig_quote_order_qty: f32,
    self_trade_prevention_mode: String,
}

pub fn f32_deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}