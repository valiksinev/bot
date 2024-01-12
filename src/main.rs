mod spot;
mod usd_futures;
mod config;

use {
    binance_spot_connector_rust::{
        http::{request::{Request, RequestBuilder}, Credentials, Method},
        hyper::{BinanceHttpClient, Error},
        market,
        trade::{self, order::{NewOrderResponseType, Side}},
    },
    rust_decimal_macros::dec,
    rust_decimal::Decimal,
    serde::Deserialize,
    // crate::book_tiker_futures::BookTickerFutures,
    core::str::FromStr,
    config::Config,
    spot::{Spot, SpotTiker, },
    usd_futures::{Futures, FuturesTiker,},
    std::{
        env, sync::Arc,
    },
};

#[tokio::main]
async fn main() -> Result<(), Error>{

    let args: Vec<String> = env::args().collect();

    let path = if args.len() == 2 {
        &args[1]
    } else {
        "/home/user/CLionProjects/edge/bot/config.json"
    };

    let config = Config::read(path)
        .unwrap_or_else(|why| panic!("Couldn't open config {}: {}", path, why));
    let config = Arc::new(config);

    let spot = Spot::new(
        Arc::new(BinanceHttpClient::with_url(&config.spot_url)),
        Arc::clone(&config),
    );

    let futures = Futures::new(
        Arc::new(BinanceHttpClient::with_url(&config.usd_futures_url)),
        Arc::clone(&config),
    );


    let (spot, futures) = futures::join!(spot.tiker(), futures.tiker());

    println!("{:?}", spot?);
    println!("{:?}\n", futures?);

    let credentials = Credentials::from_hmac(&config.api_key, &config.api_secret);


    // let price = spot.bid_price.parse::<f32>().expect(&format!("spot.bid_price parse error: {}", spot.bid_price));
    // let min : f32 = 25.0 / price;
    // let max : f32 = 100.0 / price;
    // let qty = futures.bid_qty.parse::<f32>().expect(&format!("futures.bid_qty parse error: {}", futures.bid_qty));
    //
    // let qty = if qty >= min && qty <= max {
    //     qty
    // } else {
    //     max
    // };
    //
    // let price: f32 = 30000.0;
    // let qty: f32 = 0.001;
    //
    // let client =
    //     BinanceHttpClient::with_url("https://testnet.binance.vision")
    //     // BinanceHttpClient::default()
    //         .credentials(credentials);
    //
    // let request = RequestBuilder::new(Method::Post, "/api/v3/order").params(vec![
    //     ("symbol", "BTCUSDT"),
    //     ("side", "BUY"),
    //     ("type", "LIMIT"),
    //     ("quantity", &format!("{:.8}", qty)),
    //     ("price", &format!("{:.8}", price)),
    //     ("timeInForce", "GTC"),
    // ]).sign();
    //
    // let order = client.send(request)
    //     .await?
    //     .into_body_str()
    //     .await?;
    //
    // let order : NewOrder = serde_json::from_str(&order)
    //     .expect(&format!("error to deserialize order: {}", order));
    //
    // println!("{:?}", order);
    // println!("");
    //
    // let delete = RequestBuilder::new(Method::Delete, "/api/v3/openOrders").params(vec![
    //     ("symbol", order.symbol.as_str()),
    // ]).sign();
    //
    // let delete = client.send(delete)
    //     .await?
    //     .into_body_str()
    //     .await?;
    //
    // println!("{:?}", delete);
    // println!("");
    //
    //
    // let query = RequestBuilder::new(Method::Get, "/api/v3/order").params(vec![
    //     ("symbol", order.symbol.as_str()),
    //     ("orderId", &format!("{}", order.order_id)),
    // ]).sign();
    //
    // let query = client.send(query)
    //     .await?
    //     .into_body_str()
    //     .await?;
    //
    // let query_order: QueryOrder = serde_json::from_str(&query).unwrap();
    // println!("{:?}", query_order);

    // println!("{:?}", confirm);
    // println!("");

    // let request = RequestBuilder::new(Method::Get, "/api/v3/account").sign();

    // let confirmation = client.send(request)
    //     .await?
    //     .into_body_str()
    //     .await?;

    // println!("{:?}", confirmation);


    Ok(())

}

