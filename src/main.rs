extern crate toml;
#[macro_use]
extern crate log;
extern crate env_logger;

mod shell;
mod config;

fn main() {
	env_logger::init().unwrap();

	info!("tinshel starting");
}