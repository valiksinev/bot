use serde::{
    Deserialize,
};

pub mod base;

pub use base::Spot;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTiker {
    symbol: String,
    bid_price: String,
    bid_qty: String,
    ask_price: String,
    ask_qty: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OrderStatus {
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
    price: String,
    qty: String,
    comission: String,
    comission_asset: String,
    trade_id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NewOrder {
    symbol: String,
    order_id: u32,
    order_list_id: i32,
    client_order_id: String,
    transact_time: u64,
    price: String,
    orig_qty: String,
    executed_qty: String,
    cummulative_quote_qty: String,
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
    order_id: u32,
    order_list_id: i32,
    client_order_id: String,
    price: String,
    orig_qty: String,
    executed_qty: String,
    cummulative_quote_qty: String,
    status: OrderStatus,
    time_in_force: TimeInForce,
    r#type: OrderType,
    side: Side,
    stop_price: String,
    iceberg_qty: String,
    time: u64,
    update_time: u64,
    is_working: bool,
    working_time: u64,
    orig_quote_order_qty: String,
    self_trade_prevention_mode: String,
}
