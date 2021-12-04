use std::env::var;

pub fn get() -> u16 {
	lazy_static! {
		static ref PORT: u16 = var("PORT")
			.ok()
			.and_then(|port| port.parse().ok())
			.unwrap_or(3000);
	}

	*PORT
}
