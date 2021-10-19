use actix_web::{middleware::Logger, web, App, HttpServer};
use serde_json::from_reader;
use std::fs::File;

mod app_state;
mod handlers;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on 0.0.0.0:5000");
    HttpServer::new(|| {
        App::new()
            .data(app_state::AppState {
                blotter: from_reader(File::open("data/blotter.json").unwrap()).unwrap(),
                market_data: from_reader(File::open("data/market_data.json").unwrap()).unwrap(),
            })
            .wrap(Logger::default())
            .service(
                web::scope("/")
                    .service(handlers::market_data::get_all)
                    .service(handlers::blotter::get_all)
                    .service(handlers::market_data_link::get),
            )
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
