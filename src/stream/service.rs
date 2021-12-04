use super::Stream;
use actix_web::{web::Payload, HttpRequest, Responder};
use actix_web_actors::ws::start;

pub async fn service(request: HttpRequest, stream: Payload) -> impl Responder {
	start(Stream::new(), &request, stream)
}
