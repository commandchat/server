use super::port;
use std::net::SocketAddr;

pub fn get() -> SocketAddr {
	lazy_static! {
		static ref ORIGIN: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port::get()));
	}

	*ORIGIN
}
