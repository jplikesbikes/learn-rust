#[macro_use]
extern crate log;
extern crate env_logger;
extern crate learn_rust;

use learn_rust::db;

fn main() {
	// initialize the logger implementation
	env_logger::init();

	let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

	info!("Version: {}", version);
	db::create();
}
