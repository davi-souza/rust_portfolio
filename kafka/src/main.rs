use actix_web::{get, web, App, HttpServer, Responder};
use std::str;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Message {
  a: String,
}

async fn kafka_consume() {
    let mut con = Consumer::from_hosts(vec!["kafka:9092".to_string()])
        .with_topic("topic".to_string())
        .with_group("kafka_group".to_string())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .expect("unable to create kafka con");

    loop {
        let mss = con.poll().expect("unable to poll");

        for ms in mss.iter() {
            for m in ms.messages() {
                match serde_json::from_slice::<Message>(m.value) {
                  Ok(message) => {
                    println!("new message");
                    println!("\ttopic: {}", ms.topic());
                    println!("\tpartition: {}", ms.partition());
                    println!("\toffset: {}", m.offset);
                    println!("\tmessage: {:?}", message);
                  },
                  Err(e) => {
                    println!("Error parsing message: {}", e);
                  },
                };
            }
            con.consume_messageset(ms).expect("unable to consume");
        }
        con.commit_consumed().expect("unable to commit");
    }
}

#[get("/{name}")]
async fn index(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::rt::spawn(async move { kafka_consume().await });

    HttpServer::new(|| App::new().service(index))
        .workers(1)
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
