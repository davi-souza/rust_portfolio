use super::pair::Pair;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Trade {
    pub id: i32,
    pub pair: Pair,
    pub timestamp: NaiveDateTime,
    pub qty: i32,
    pub price: i32,
}

#[derive(Debug, PartialEq)]
pub struct TradeFilter {
    pub pairs: Option<Vec<Pair>>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub min_qty: Option<i32>,
    pub max_qty: Option<i32>,
}

pub fn get_trades(blotter: &[Trade], filter: TradeFilter) -> Vec<&Trade> {
    blotter
        .iter()
        .filter(|&t| {
            (match &filter.pairs {
                Some(pairs) => pairs.contains(&t.pair),
                None => true,
            }) && (match filter.start_date {
                Some(date) => t.timestamp >= date,
                None => true,
            }) && (match filter.end_date {
                Some(date) => t.timestamp <= date,
                None => true,
            }) && (match filter.min_price {
                Some(price) => t.price >= price,
                None => true,
            }) && (match filter.max_price {
                Some(price) => t.price <= price,
                None => true,
            }) && (match filter.min_qty {
                Some(qty) => t.qty >= qty,
                None => true,
            }) && (match filter.max_qty {
                Some(qty) => t.qty <= qty,
                None => true,
            })
        })
        .collect()
}

pub fn get_trade(blotter: &[Trade], trade_id: i32) -> Option<&Trade> {
    blotter.iter().find(|t| t.id == trade_id)
}
