#[macro_use]
extern crate lazy_static;

mod port;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use port::PORT;
use std::io;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("commandchat")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind(format!("localhost:{}", *PORT))?
        .run()
        .await
}
