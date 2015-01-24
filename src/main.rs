#[macro_use] extern crate log;
extern crate toml;
use std::io::{File};

fn main() {
	info!("tinshel starting");
	debug!("now load config");
	let configpath = Path::new("/home/liam/.tinshel.toml");


	let configcontents = match File::open(&configpath).read_to_string() {
        Ok(s) => s,
        Err(e) => panic!("error reading file: {}", e),
    };

    //println!("{}", configcontents);
	let config = toml::Parser::new(configcontents.as_slice()).parse().unwrap();
	debug!("{:?}", config);
}