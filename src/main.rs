// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
extern crate env_logger;
extern crate learn_rust;

use learn_rust::db;
use errors::*;

mod errors {
	// Create the Error, ErrorKind, ResultExt, and ResultTypes
	error_chain!{}
}

quick_main!(run);

fn run() -> Result<()> {
	let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

	info!("Version: {}", version);
	db::create();

	use std::fs::File;

	// This operation will fail
	File::open("contacts").chain_err(|| "unable to open contacts file")?;

	Ok(())
}
