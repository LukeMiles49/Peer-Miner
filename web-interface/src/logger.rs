use wasm_bindgen::prelude::*;

use std::sync::{
	Arc,
	Mutex,
};

use lib::FnWriter;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(msg: &str);
	
	#[wasm_bindgen(js_namespace = console)]
	fn warn(msg: &str);
}

fn line_buffered_fn_writer<F: Fn(&str) + Send + Sync>(handler: F) -> FnWriter<impl FnMut(&[u8]) + Send> {
	let partial_line = Arc::new(Mutex::new(String::new()));
	FnWriter::new(move |buf| {
		let mut partial_line = partial_line.lock().unwrap();
		match String::from_utf8(Vec::from(buf)) {
			Ok(msg) => {
				let msg = msg + "\n"; // Add a newline because lines doesn't return the last line if empty
				let mut lines = msg.lines();
				*partial_line += lines.next().unwrap();
				for line in lines {
					handler(&partial_line);
					*partial_line = String::from(line);
				}
			},
			Err(_) => (),
		}
	})
}

pub fn bind_panics() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));
	
	std::io::set_panic(Some(Box::new(line_buffered_fn_writer(warn))));
}

pub fn bind_loggers() {
	std::io::set_print(Some(Box::new(line_buffered_fn_writer(log))));
}
