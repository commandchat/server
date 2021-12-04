use super::port;
use std::net::SocketAddr;

pub fn get() -> SocketAddr {
	lazy_static! {
		static ref ORIGIN: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port::get()));
	}

	*ORIGIN
}
