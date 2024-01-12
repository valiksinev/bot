
use {
    binance_spot_connector_rust::{
        hyper::{BinanceHttpClient, Error}, market,
    },
    hyper::client::connect::Connect,
    std::sync::Arc,
    crate::config::Config,
};
use crate::spot::SpotTiker;


pub struct Spot<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    pub client: Arc<BinanceHttpClient<T>>,
    pub config: Arc<Config>,
}

impl<T> Spot<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    pub fn new(client: Arc<BinanceHttpClient<T>>, config: Arc<Config> ) -> Spot<T> {
        Spot {
            client,
            config,
        }
    }

    pub async fn tiker (&self) -> Result<SpotTiker, Error> {
        let request = market::book_ticker().symbol(&self.config.symbol);
        let response = self.client.send(request).await?.into_body_str().await;
        let tiker = serde_json::from_str(&response?)
            .expect("Couldn't parse response GET /api/v3/ticker/bookTicker");

        Ok(tiker)
    }

    // pub fn limit_order(&self, ) {
    //
    // }
}