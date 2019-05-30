extern crate rust_nao;

use rust_nao::{Handler, Sauce};

const FILE : &str = "https://i.imgur.com/W42kkKS.jpg";

fn create_handler() -> Handler {
	let data = std::fs::read_to_string("config.json").expect("Couldn't read file.");
	let json : serde_json::Value = serde_json::from_str(data.as_str()).expect("JSON not well formatted.");
	let api_key = json["api_key"].as_str();

	match api_key {
		Some(key) => {
			Handler::new(key, 0, [].to_vec(), [].to_vec(), 999, 999)
		},
		None => {
			Handler::new("", 0, [].to_vec(), [].to_vec(), 999, 999)
		}
	}
}

#[test]
fn check_handler_creation() {
	create_handler();
}

#[test]
fn try_get_sauce() {
	let mut handle = create_handler();
	let vec = handle.get_sauce(FILE).unwrap();
	for v in vec {
		println!("{:?}", v);
	}
}

#[test]
fn try_get_sauce_as_json() {
	let mut handle = create_handler();
	handle.get_sauce_as_json(FILE).unwrap();
}

#[test]
fn get_short_limits() {
	let mut handle = create_handler();
	handle.get_short_limit();
	handle.get_current_short_limit();
	handle.get_sauce(FILE).unwrap();
	handle.get_short_limit();
	handle.get_current_short_limit();
}

#[test]
fn get_long_limits() {
	let mut handle = create_handler();
	handle.get_long_limit();
	handle.get_current_long_limit();
	handle.get_sauce(FILE).unwrap();
	handle.get_long_limit();
	handle.get_current_long_limit();
}

#[test]
fn filter_empty_sauce() {
	let mut handle = create_handler();
	let vec : Vec<Sauce> = handle.get_sauce(FILE).unwrap();
	let _only_empty : Vec<Sauce> = vec.into_iter().filter(|sauce| !sauce.has_empty_url()).collect();
}