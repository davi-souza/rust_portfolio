use super::{auth_user::AuthUser, Data};
use crate::app_state::AppState;
use crate::models::trade::{get_trades, TradeFilter};
use crate::utils::parse_pair_array;
use actix_web::{get, web, HttpResponse, Result};
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::fmt::Debug;

#[get("/blotter")]
pub async fn get_all(
    _user: AuthUser,
    state: web::Data<AppState>,
    query: web::Query<BlotterQueryParams>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Data {
        data: get_trades(&state.blotter, TradeFilter::from(query.into_inner())),
    }))
}

#[derive(Debug, Deserialize)]
pub struct BlotterQueryParams {
    pub pairs: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub min_qty: Option<i32>,
    pub max_qty: Option<i32>,
}

impl From<BlotterQueryParams> for TradeFilter {
    fn from(params: BlotterQueryParams) -> Self {
        Self {
            pairs: parse_pair_array(params.pairs),
            start_date: match params.start_date {
                Some(date) => match date.parse::<NaiveDateTime>() {
                    Ok(naive_date) => Some(naive_date),
                    Err(_) => None,
                },
                None => None,
            },
            end_date: match params.end_date {
                Some(date) => match date.parse::<NaiveDateTime>() {
                    Ok(naive_date) => Some(naive_date),
                    Err(_) => None,
                },
                None => None,
            },
            min_price: params.min_price,
            max_price: params.max_price,
            min_qty: params.min_qty,
            max_qty: params.max_qty,
        }
    }
}
