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
    std::sync::Arc,
};





#[tokio::main]
async fn main() -> Result<(), Error>{

    let config = Config::read("/home/user/CLionProjects/edge/bot/config.json").unwrap();

    let spot = Spot::new(Arc::new(BinanceHttpClient::with_url(&config.spot_url)));
    let futures = Spot::new(Arc::new(BinanceHttpClient::with_url(&config.usd_futures_url)));


    let f1 = async {
        let request = market::book_ticker().symbol("BTCUSDT");
        spot.client.send(request).await?.into_body_str().await
    };

    let f2 = async {
        let request = RequestBuilder::new(Method::Get, "/fapi/v1/ticker/bookTicker")
            .params(vec![
                ("symbol", "BTCUSDT")
            ]);

        futures.client.send(request).await?.into_body_str().await
    };


    let (spot, futures) = futures::join!(f1, f2);
    let spot: SpotTiker = serde_json::from_str(&spot?).unwrap();
    let futures: FuturesTiker = serde_json::from_str(&futures?).unwrap();

    println!("{:?}", spot);
    println!("{:?}\n", futures);
    //
    //
    // let credentials = Credentials::from_hmac(
    //     "AgMzxt1KB7QtrnwB8QdriqKCVHsZyyVZSXeun29htPB68yfSKnOj07NWXC0OgQwl".to_owned(),
    //     "W7fhQZtB3xL4sml3WpLxILdBSqIPCuZwP71bg8WFm5AIsHJ1hNKtwuvXFfbV0m6h".to_owned()
    // );
    //
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

