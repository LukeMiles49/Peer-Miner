use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod game_impl;
use game_impl::*;

#[cfg(debug_assertions)]
use {
	console_error_panic_hook,
	std::panic,
};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Assume environment is owned by the JavaScript environment, and ownership is passed implicitly
// by any JavaScript to WebAssembly call
static mut ENV: Option<Environment> = None;

pub struct Environment {
	game: Game,
}

impl Environment {
	// TODO: Run-time checks?
	
	pub fn take_ownership() -> &'static mut Self {
		// JS implicitly passes ownership of the environment
		unsafe { ENV.as_mut().unwrap() }
	}
	
	pub fn init(env: Option<Environment>) {
		unsafe { ENV = env; }
	}
}

#[wasm_bindgen(start)]
pub fn start() {
	#[cfg(debug_assertions)]
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	
	let window = web_sys::window()
		.unwrap();
	let document = window
		.document()
		.unwrap();
	let canvas = document
		.get_element_by_id("game-canvas")
		.unwrap()
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.unwrap();
	let context = canvas
		.get_context("2d")
		.unwrap()
		.unwrap()
		.dyn_into::<web_sys::CanvasRenderingContext2d>()
		.unwrap();
	
	Environment::init(Some(Environment {
		game: Game::new(
			WebTimer::new(),
			WebCanvas::new(context),
		),
	}));
	
	let env = Environment::take_ownership();
	env.game.start();
}
