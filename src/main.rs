#[macro_use]
extern crate lazy_static;

mod origin;
mod stream;

use actix_files::Files;
use actix_web::{guard, web, App, HttpServer};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.route(
				"/",
				web::get()
					.guard(guard::Header("upgrade", "websocket"))
					.to(stream::service),
			)
			.service(Files::new("/", "public").index_file("index.html"))
	})
	.bind(origin::get())?
	.run()
	.await
}
