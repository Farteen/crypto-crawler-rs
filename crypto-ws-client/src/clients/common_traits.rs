// tick-by-tick trade
pub(super) trait Trade {
    fn subscribe_trade(&mut self, pairs: &[String]);
}

// 24hr rolling window ticker
pub(super) trait Ticker {
    fn subscribe_ticker(&mut self, pairs: &[String]);
}

// Best Bid & Offer
pub(super) trait BBO {
    fn subscribe_bbo(&mut self, pairs: &[String]);
}

// An orderbook snapshot followed by realtime updates.
pub(super) trait OrderBook {
    fn subscribe_orderbook(&mut self, pairs: &[String]);
}

pub trait OrderBookSnapshot {
    /// Subscribes to orderbook snapshot channels.
    ///
    /// A orderbook snapshot channel sends a complete snapshot every interval.
    ///
    /// This function subscribes to exchange specific channels as the following:
    ///
    /// * Binance `depth10`, every 100ms
    /// * Bitfinex `xxx` channel with `prec=P0`, `frec=F0` and `len=25`
    /// * BitMEX `orderBook10`
    /// * Bitstamp `xxx`
    /// * CoinbasePro `xxx`
    /// * Huobi `xxxx` for contracts, `mbp.20` for Spot
    /// * Kraken `xxx` with `depth=25`
    /// * MXC `depth.full` for Swap, `symbol` for Spot
    /// * OKEx `depth5`
    fn subscribe_orderbook_snapshot(&mut self, pairs: &[String]);
}

macro_rules! impl_trait {
    ($trait_name:ident, $struct_name:ident, $method_name:ident, $channel_name:expr, $to_raw_channel: ident) => {
        impl<'a> $trait_name for $struct_name<'a> {
            fn $method_name(&mut self, pairs: &[String]) {
                let pair_to_raw_channel = |pair: &String| $to_raw_channel($channel_name, pair);

                let channels = pairs
                    .iter()
                    .map(pair_to_raw_channel)
                    .collect::<Vec<String>>();
                self.client.subscribe(&channels);
            }
        }
    };
}
