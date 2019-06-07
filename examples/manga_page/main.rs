//! A simple example, assuming you had a config.json file that had your api key.

extern crate rustnao;
use rustnao::{Handler, Sauce};

fn main() {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();
	let file = "https://s5.mangadex.org/data/f2cf04ff9fbd5374c20d1cd5a555efed/x2.png";

	match api_key {
		Some(key) => {
			let mut handle = Handler::new(key, Some(0), None, None, Some(999), Some(999));
			let result : Vec<Sauce> = handle.get_sauce(file).unwrap();
			for i in result {
				println!("{:?}", i);
			}
		},
		None => (),
	}
}