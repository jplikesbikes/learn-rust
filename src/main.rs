extern crate learn_rust;

use learn_rust::db;

fn main() {
	let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

	println!("Version: {}", version);
	db::create();
}
