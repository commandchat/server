use std::env::var;

lazy_static! {
	pub static ref PORT: String = var("PORT").unwrap_or(String::from("3000"));
}
