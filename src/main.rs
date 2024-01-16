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
    usd_futures::{Futures, LocalSpawner},
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

    let spot_credentials = Credentials::from_hmac(&config.spot_api_key, &config.spot_api_secret);
    let futures_credentials = Credentials::from_hmac(&config.futures_api_key, &config.futures_api_secret);

    let spot = Spot::new(
        Arc::new(
            BinanceHttpClient::with_url(&config.spot_url).
                credentials(spot_credentials.clone())
        ),
        Arc::clone(&config),
    );

    let futures = Futures::new(
        Arc::new(
            BinanceHttpClient::with_url(&config.usd_futures_url)
                .credentials(futures_credentials.clone())
        ),
        Arc::clone(&config),
    );

    let (spot_ticker, futures_ticker) = futures::join!(spot.ticker(), futures.ticker());
    let (spot_ticker, futures_ticker) = (spot_ticker?, futures_ticker?);

    println!("{:?}", spot_ticker);
    println!("{:?}\n", futures_ticker);

    let local_spawner = LocalSpawner::new(Arc::new(futures));

    let total = config.total_input_amount as f32;
    let  mut spent = 0.0;
    while spent < total {
        spent += spot.limit_order(&spot_ticker, &futures_ticker, &local_spawner).await?;
        println!("spent: {}, remained : {}", spent, total - spent);
    }

    local_spawner.join();
    Ok(())
}

