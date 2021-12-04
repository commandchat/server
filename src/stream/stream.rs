use actix::{prelude::*, Actor, StreamHandler};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use std::time::{Duration, Instant};

const PING_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Stream {
	last_ping: Instant,
}

impl Stream {
	pub fn new() -> Self {
		Self {
			last_ping: Instant::now(),
		}
	}
}

impl Actor for Stream {
	type Context = WebsocketContext<Self>;

	fn started(&mut self, context: &mut Self::Context) {
		context.run_interval(PING_INTERVAL, |stream, context| {
			if Instant::now().duration_since(stream.last_ping) > CLIENT_TIMEOUT {
				context.stop();
			} else {
				context.ping(b"");
			}
		});
	}
}

impl StreamHandler<Result<Message, ProtocolError>> for Stream {
	fn handle(&mut self, message: Result<Message, ProtocolError>, context: &mut Self::Context) {
		match message {
			Ok(Message::Ping(message)) => {
				self.last_ping = Instant::now();
				context.pong(&message);
			}
			Ok(Message::Pong(_)) => {
				self.last_ping = Instant::now();
			}
			Ok(Message::Text(message)) => context.text(message),
			Ok(Message::Binary(message)) => context.binary(message),
			Ok(Message::Close(reason)) => {
				context.close(reason);
				context.stop();
			}
			_ => context.stop(),
		}
	}
}
