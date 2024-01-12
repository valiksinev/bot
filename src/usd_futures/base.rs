
use {
    binance_spot_connector_rust::{
        hyper::{BinanceHttpClient, Error}, http::{request::RequestBuilder, Method},
    },
    std::sync::Arc,
    hyper::client::connect::Connect,
    crate::config::Config,
    super::FuturesTicker,
};


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
        let response = self.client.send(request).await?.into_body_str().await;
        let ticker = serde_json::from_str(&response?)
            .expect("Couldn't parse response GET /fapi/v1/ticker/bookTicker");

        Ok(ticker)
    }


}