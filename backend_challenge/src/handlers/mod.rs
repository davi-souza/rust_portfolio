use serde::Serialize;

pub mod auth_user;
pub mod blotter;
pub mod market_data;
pub mod market_data_link;

#[derive(Serialize)]
pub struct Data<T> {
    data: T,
}
