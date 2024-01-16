use {
    binance_spot_connector_rust::{
        hyper::{BinanceHttpClient, Error}, market,
        http::{Method, request::RequestBuilder, Credentials},
    },
    crate::{
        config::Config, usd_futures::{FuturesTicker, LocalSpawner, Task},
    },
    hyper::{
        client::connect::Connect,
    },
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
        let response = self.client.send(request).await?.into_body_str().await?;
        let ticker = serde_json::from_str(&response)
            .expect(&format!("Couldn't parse response GET /api/v3/ticker/bookTicker: {}", &response));

        Ok(ticker)
    }

    pub async fn limit_order(
        &self,
        spot_ticker: &SpotTicker,
        futures_ticker: &FuturesTicker,
        spawner: &LocalSpawner,
    ) -> Result<(), Error> {

        let min : f32 = self.config.min_order_size as f32 / spot_ticker.bid_price;
        let max : f32 = self.config.max_order_size as f32 / spot_ticker.bid_price;

        let qty = if futures_ticker.bid_qty >= min && futures_ticker.bid_qty <= max {
            futures_ticker.bid_qty
        } else {
            max
        };

        // TODO: remove this workaround
        let price: f32 = 43600.0;
        let qty: f32 = 0.001;

        println!("create spot limit order");
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

        let response = match self.client.send(request).await {
            Ok(mes) => {
                mes.into_body_str().await.unwrap()
            },
            Err(why) => {
                println!("Couldn't send Spot limit order: {:?}", why);
                return Err(why);
            }
        };

        let order : LimitOrder = serde_json::from_str(&response)
            .expect(&format!("Couldn't deserialize response Post /api/v3/order: {}", response));

        println!("{:?}\n", order);

        let mut executed_qty = 0_f32;

        while executed_qty < order.orig_qty {
            let query_order = self.query_order(order.order_id).await?;
            let qty = query_order.executed_qty - executed_qty;

            spawner.spawn(Task::market_order(qty));
            executed_qty = query_order.executed_qty;
            println!("limit order executed: {}", executed_qty);
        }
        println!("finished");

        Ok(())
    }


    async fn query_order(&self, order_id: u64) -> Result<(QueryOrder), Error> {
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

        Ok(query_order)
    }

    pub async fn all_orders(&self) -> Result<(), Error> {
        let request = RequestBuilder::new(Method::Get, "/api/v3/allOrders")
            .params(vec![
                ("symbol", self.config.symbol.as_str()),
            ])
            .sign();

        let query = self.client.send(request)
            .await?
            .into_body_str()
            .await?;

        println!("{:?}\n", query);

        Ok(())
    }
}