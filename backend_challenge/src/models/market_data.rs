use super::{pair::Pair, provider::Provider};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketData {
    pub id: i32,
    pub provider: Provider,
    pub pair: Pair,
    pub timestamp: NaiveDateTime,
    pub feed_price: i32,
    pub vwap_price: i32,
    pub overall_price: i32,
}

#[derive(Debug)]
pub struct MarketDataFilter {
    pub providers: Option<Vec<Provider>>,
    pub pair: Option<Pair>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

pub fn get_market_data(market_data: &[MarketData], filter: MarketDataFilter) -> Vec<&MarketData> {
    market_data
        .iter()
        .filter(|md| {
            (match &filter.providers {
                Some(providers) => providers.contains(&md.provider),
                None => true,
            }) && (match &filter.pair {
                Some(pair) => &md.pair == pair,
                None => true,
            }) && (match filter.start_date {
                Some(date) => md.timestamp >= date,
                None => true,
            }) && (match filter.end_date {
                Some(date) => md.timestamp <= date,
                None => true,
            })
        })
        .collect()
}
