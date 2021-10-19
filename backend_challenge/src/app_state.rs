use crate::models::{market_data::MarketData, trade::Trade};

pub struct AppState {
    pub blotter: Vec<Trade>,
    pub market_data: Vec<MarketData>,
}
