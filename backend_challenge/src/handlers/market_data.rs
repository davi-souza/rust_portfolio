use super::{auth_user::AuthUser, Data};
use crate::app_state::AppState;
use crate::models::market_data::{get_market_data, MarketDataFilter};
use crate::utils::parse_provider_array;
use actix_web::{get, web, HttpResponse, Result};
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::fmt::Debug;

#[get("/market_data")]
pub async fn get_all(
    _user: AuthUser,
    state: web::Data<AppState>,
    query: web::Query<MarketDataQueryParams>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Data {
        data: get_market_data(
            &state.market_data,
            MarketDataFilter::from(query.into_inner()),
        ),
    }))
}

#[derive(Debug, Deserialize)]
pub struct MarketDataQueryParams {
    pub providers: Option<String>,
    pub pair: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl From<MarketDataQueryParams> for MarketDataFilter {
    fn from(params: MarketDataQueryParams) -> Self {
        Self {
            providers: parse_provider_array(params.providers),
            pair: match params.pair {
                Some(pair) => serde_json::from_str(&format!(r#""{}""#, &pair)).unwrap(),
                None => None,
            },
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
        }
    }
}
