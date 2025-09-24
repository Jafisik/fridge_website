use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("100.81.131.123:8080")?
        .run()
        .await
}