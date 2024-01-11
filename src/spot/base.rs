
use {
    binance_spot_connector_rust::hyper::{
        BinanceHttpClient,
    },
    std::sync::Arc,
};
use hyper::client::connect::Connect;


pub struct Spot<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    pub client: Arc<BinanceHttpClient<T>>,
}

impl<T> Spot<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    pub fn new(client: Arc<BinanceHttpClient<T>> ) -> Spot<T> {
        Spot {
            client
        }
    }
}