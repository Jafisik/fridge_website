use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rusqlite::{Connection, params};
use serde::Deserialize;

#[derive(Deserialize)]
struct Item {
    name: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("index.html"))
}

async fn add_item(item: web::Json<Item>) -> impl Responder {
    let conn = Connection::open("fridge.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        [],
    ).unwrap();

    conn.execute(
        "INSERT INTO items (name) VALUES (?1)",
        params![item.name],
    ).unwrap();

    HttpResponse::Ok().body("Uloženo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/add_item", web::post().to(add_item))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
