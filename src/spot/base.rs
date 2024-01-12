
use {
    binance_spot_connector_rust::{
        hyper::{BinanceHttpClient, Error}, market,
        http::{Method, request::RequestBuilder},
    },
    crate::config::Config,
    crate::usd_futures::FuturesTicker,
    hyper::client::connect::Connect,
    std::sync::Arc,
    super::{SpotTicker, LimitOrder, QueryOrder},
};


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

    pub async fn ticker (&self) -> Result<SpotTicker, Error> {
        let request = market::book_ticker().symbol(&self.config.symbol);
        let response = self.client.send(request).await?.into_body_str().await;
        let ticker = serde_json::from_str(&response?)
            .expect("Couldn't parse response GET /api/v3/ticker/bookTicker");

        Ok(ticker)
    }

    pub async fn limit_order(&self, spot_ticker: &SpotTicker, futures_ticker: &FuturesTicker) -> Result<(), Error> {

        let min : f32 = self.config.min_order_size as f32 / spot_ticker.bid_price;
        let max : f32 = self.config.max_order_size as f32 / spot_ticker.bid_price;

        let qty = if futures_ticker.bid_qty >= min && futures_ticker.bid_qty <= max {
            futures_ticker.bid_qty
        } else {
            max
        };

        // TODO: remove this workaround
        let price: f32 = 30000.0;
        let qty: f32 = 0.001;


        let request = RequestBuilder::new(Method::Post, "/api/v3/order")
            .params(vec![
                ("symbol", self.config.symbol.as_str()),
                ("side", "BUY"),
                ("type", "LIMIT"),
                ("quantity", &format!("{:.8}", qty)),
                ("price", &format!("{:.8}", price)),
                ("timeInForce", "GTC"),
            ])
            .sign();

        let response = self.client.send(request)
            .await?
            .into_body_str()
            .await?;

        let order : LimitOrder = serde_json::from_str(&response)
            .expect(&format!("Couldn't deserialize response Post /api/v3/order: {}", response));

        println!("{:?}\n", order);

        self.query_order(order.order_id).await?;
        self.delete_open_orders().await?;

        Ok(())
    }

    /// debug only
    async fn delete_open_orders(&self) -> Result<(), Error> {
        let request = RequestBuilder::new(Method::Delete, "/api/v3/openOrders")
            .params(vec![
                ("symbol", self.config.symbol.as_str()),
            ])
            .sign();

        let delete = self.client.send(request)
            .await?
            .into_body_str()
            .await?;

        println!("deleted orders: {:?}\n", delete);

        Ok(())
    }

    async fn query_order(&self, order_id: u32) -> Result<(), Error> {
        let request = RequestBuilder::new(Method::Get, "/api/v3/order")
            .params(vec![
                ("symbol", self.config.symbol.as_str()),
                ("orderId", &format!("{order_id}")),
            ])
            .sign();

        let query = self.client.send(request)
            .await?
            .into_body_str()
            .await?;

        let query_order: QueryOrder = serde_json::from_str(&query)
            .expect(&format!("Couldn't deserialize response Get /api/v3/order: {}", query));

        println!("{:?}\n", query_order);

        Ok(())
    }
}