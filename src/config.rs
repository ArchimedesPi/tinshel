use std::io;
use std::io::prelude::*;
use std::fs::File;
use toml;

pub struct Config {
	prompt: String
}

impl Config {
	fn new(config_str: &str) -> Config {
		Config {
			prompt: "> ".to_string()

		}
	}
	fn load_options(&mut self, options_str: &str) -> Result<Config, ConfigError> {
		let parsed_toml = toml::Parser::new(options_str).parse().unwrap();
		debug!("parsed toml {:?}", parsed_toml);

		Ok(Config { 
			prompt: "foo".to_string()
		})
	}

	fn load_options_from_file(&mut self, options_path: &str) -> Result<Config, ConfigError> {
		let mut file = try!(File::open(options_path));

		let mut options_str = String::new();
		try!(file.read_to_string(&mut options_str));

		try!(self.load_options(&mut options_str))
	}
}