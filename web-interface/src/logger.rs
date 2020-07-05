use game_interface::{
	Logger,
	LogLevel,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(msg: &str);
}

pub struct WebLogger { }

impl Logger for WebLogger {
	const VERBOSITY: LogLevel =
		if cfg!(feature = "debug") { LogLevel::Debug }
		else { LogLevel::Warning };
	
	fn print(msg: &str) {
		log(msg);
	}
}
