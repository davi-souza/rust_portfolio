mod domain;
mod repository;
mod utils;

use crate::domain::*;
use crate::repository::ItemRepository;
use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use failure::Error;
use r2d2_sqlite::SqliteConnectionManager;
use std::io;
use uuid::Uuid;

type Pool = r2d2::Pool<SqliteConnectionManager>;

async fn get_all(db: web::Data<Pool>) -> Result<HttpResponse, AWError> {
    Ok(web::block::<_, _, Error>(move || {
        let conn = db.get()?;
        let item_repo = ItemRepository::new(&conn);
        item_repo.get_all()
    })
    .await
    .map(|items| HttpResponse::Ok().json(items))
    .map_err(|e| {
        println!("{:?}", e);
        HttpResponse::InternalServerError()
    })?)
}

async fn get_by_id(
    db: web::Data<Pool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, AWError> {
    Ok(web::block::<_, _, Error>(move || {
        let conn = db.get()?;
        let item_repo = ItemRepository::new(&conn);
        let id = Uuid::parse_str(&path.into_inner().0).unwrap();
        item_repo.get_by_id(id)
    })
    .await
    .map(|items| HttpResponse::Ok().json(items))
    .map_err(|e| {
        println!("{:?}", e);
        HttpResponse::InternalServerError()
    })?)
}

async fn get_all_history(db: web::Data<Pool>) -> Result<HttpResponse, AWError> {
    Ok(web::block::<_, _, Error>(move || {
        let conn = db.get()?;
        let item_repo = ItemRepository::new(&conn);
        item_repo.get_all_internal()
    })
    .await
    .map(|items| HttpResponse::Ok().json(items))
    .map_err(|e| {
        println!("{:?}", e);
        HttpResponse::InternalServerError()
    })?)
}

async fn get_history_by_id(
    db: web::Data<Pool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, AWError> {
    Ok(web::block::<_, _, Error>(move || {
        let conn = db.get()?;
        let item_repo = ItemRepository::new(&conn);
        let id = Uuid::parse_str(&path.into_inner().0).unwrap();
        item_repo.get_all_internal_by_id(id)
    })
    .await
    .map(|items| HttpResponse::Ok().json(items))
    .map_err(|e| {
        println!("{:?}", e);
        HttpResponse::InternalServerError()
    })?)
}

async fn create_one(
    db: web::Data<Pool>,
    new_item: web::Json<NewItem>,
) -> Result<HttpResponse, AWError> {
    Ok(web::block::<_, _, Error>(move || {
        let conn = db.get()?;
        let item_repo = ItemRepository::new(&conn);
        item_repo.create_one(new_item.into_inner())
    })
    .await
    .map(|items| HttpResponse::Ok().json(items))
    .map_err(|e| {
        println!("{:?}", e);
        HttpResponse::InternalServerError()
    })?)
}

async fn update_by_id(
    db: web::Data<Pool>,
    path: web::Path<(String,)>,
    update_item: web::Json<UpdateItem>,
) -> Result<HttpResponse, AWError> {
    Ok(web::block::<_, _, Error>(move || {
        let conn = db.get()?;
        let item_repo = ItemRepository::new(&conn);
        let id = Uuid::parse_str(&path.into_inner().0).unwrap();
        item_repo.update_one(id, update_item.into_inner())
    })
    .await
    .map(|item| HttpResponse::Ok().json(item))
    .map_err(|e| {
        println!("{:?}", e);
        HttpResponse::InternalServerError()
    })?)
}

// async fn delete_db_id(db: web::Data<Pool>) -> Result<HttpResponse, AWError> {}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = SqliteConnectionManager::file("app.db");
    let pool = Pool::new(manager).unwrap();

    let db = pool.get().unwrap();
    db.execute(
        r#"
        create table if not exists "item" (
            "pk" string primary key,
            "id" string not null,
            "text" string not null,
            "number" integer not null,
            "created_at" string not null,
            "revoked_at" string null
        )
        "#,
        [],
    )
    .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/history").route(web::get().to(get_all_history)))
            .service(web::resource("/history/{id}").route(web::get().to(get_history_by_id)))
            .service(
                web::resource("/")
                    .route(web::get().to(get_all))
                    .route(web::post().to(create_one)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_by_id))
                    .route(web::put().to(update_by_id)),
            )
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
