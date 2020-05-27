#[cfg(debug_assertions)]
extern crate console_error_panic_hook;

#[cfg(debug_assertions)]
use std::panic;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
	#[cfg(debug_assertions)]
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	
	web_sys::console::log_1(&"Hello, World!".into());
}
