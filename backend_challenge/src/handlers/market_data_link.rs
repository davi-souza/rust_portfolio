use super::{auth_user::AuthUser, Data};
use crate::app_state::AppState;
use crate::models::market_data::{get_market_data, MarketData, MarketDataFilter};
use crate::models::trade::{get_trade, Trade};
use crate::utils::parse_provider_array;
use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[get("/market_data_link")]
pub async fn get(
    _user: AuthUser,
    state: web::Data<AppState>,
    query: web::Query<MarketDataLinkQueryParams>,
) -> Result<HttpResponse> {
    let query_params = query.into_inner();
    let maybe_trade = get_trade(&state.blotter, query_params.trade_id);
    Ok(HttpResponse::Ok().json(Data {
        data: match maybe_trade {
            Some(trade) => {
                let filter = MarketDataFilter {
                    providers: parse_provider_array(query_params.providers),
                    pair: Some(trade.pair),
                    start_date: None,
                    end_date: None,
                };
                MarketDataLink {
                    trade: Some(trade),
                    market_data: Some(get_market_data(&state.market_data, filter)),
                }
            }
            None => MarketDataLink {
                trade: None,
                market_data: None,
            },
        },
    }))
}

#[derive(Debug, Deserialize)]
pub struct MarketDataLinkQueryParams {
    pub trade_id: i32,
    pub providers: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MarketDataLink<'a> {
    pub trade: Option<&'a Trade>,
    pub market_data: Option<Vec<&'a MarketData>>,
}
