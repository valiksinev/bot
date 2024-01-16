
use {
    binance_spot_connector_rust::{
        hyper::{BinanceHttpClient, Error}, http::{request::RequestBuilder, Method},
    },
    std::sync::Arc,
    hyper::client::connect::Connect,
    crate::config::Config,
    super::{FuturesTicker, MarketOrder},
    std::{
        marker::Send, time::{SystemTime, UNIX_EPOCH},
    },

};

use hyper::{client::HttpConnector};
use hyper_tls::HttpsConnector;

pub struct Futures<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    pub client: Arc<BinanceHttpClient<T>>,
    pub config: Arc<Config>,
}

impl<T> Futures<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    pub fn new(client: Arc<BinanceHttpClient<T>>, config: Arc<Config> ) -> Futures<T> {
        Futures {
            client,
            config,
        }
    }

    pub async fn ticker (&self) -> Result<FuturesTicker, Error> {
        let request = RequestBuilder::new(Method::Get, "/fapi/v1/ticker/bookTicker")
            .params(vec![
                ("symbol", self.config.symbol.as_str())
            ]);
        let response = self.client.send(request).await?.into_body_str().await?;
        let ticker = serde_json::from_str(&response)
            .expect(&format!("Couldn't parse response GET /fapi/v1/ticker/bookTicker: {}", &response));

        Ok(ticker)
    }

    pub async fn market_order(&self, qty: f32) {
        // TODO: remove this workaround
        let qty = 0.01_f32;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Clock may have gone backwards")
            .as_millis();

        let request = RequestBuilder::new(Method::Post, "/fapi/v1/order")
            .params(vec![
                ("symbol", self.config.symbol.as_str()),
                ("side", "SELL"),
                ("type", "MARKET"),
                ("quantity", &format!("{:.8}", qty)),
                ("timestamp", &timestamp.to_string()),
            ])
            .sign();


        let response = match self.client.send(request).await {
            Ok(mes) => {
                mes.into_body_str().await.unwrap()
            },
            Err(why) => {
                println!("Couldn't send USD-Futures market order: {:?}", why);
                return;
            }
        };
        // let response = self.client.send(request).await.unwrap().into_body_str().await.unwrap();
        let order: MarketOrder = serde_json::from_str(&response)
            .expect(&format!("Couldn't parse response POST /fapi/v1/order: {}", &response));

        println!("{:?}", order);
    }
}