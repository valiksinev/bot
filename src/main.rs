mod spot;
mod usd_futures;
mod config;

use {
    binance_spot_connector_rust::{
        hyper::{BinanceHttpClient, Error}, http::Credentials,
    },
    serde::Deserialize,
    core::str::FromStr,
    config::Config,
    spot::{Spot,  },
    usd_futures::{Futures,},
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

    let credentials = Credentials::from_hmac(&config.api_key, &config.api_secret);

    let spot = Spot::new(
        Arc::new(
            BinanceHttpClient::with_url(&config.spot_url).
                credentials(credentials.clone())
        ),
        Arc::clone(&config),
    );

    let futures = Futures::new(
        Arc::new(
            BinanceHttpClient::with_url(&config.usd_futures_url)
                .credentials(credentials)
        ),
        Arc::clone(&config),
    );

    let (spot_ticker, futures_ticker) = futures::join!(spot.ticker(), futures.ticker());
    let (spot_ticker, futures_ticker) = (spot_ticker?, futures_ticker?);

    println!("{:?}", spot_ticker);
    println!("{:?}\n", futures_ticker);

    spot.limit_order(&spot_ticker, &futures_ticker).await?;

    Ok(())
}

