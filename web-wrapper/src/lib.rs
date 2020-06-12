use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_interface::{
	Environment,
	WebCanvas,
	WebTimer,
};

#[cfg(debug_assertions)]
use {
	console_error_panic_hook,
	std::panic,
};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Game = <Env as Environment>::TState;

// Assume environment is owned by the JavaScript environment, and ownership is passed implicitly
// by any JavaScript to WebAssembly call
static mut ENV: Option<Env> = None;

struct Env {
	game: Game,
}

impl Env {
	pub fn init(env: Option<Env>) {
		unsafe { ENV = env; }
	}
}

impl Environment for Env {
	type TState = game_client::Game<
		WebTimer<Self>,
		WebCanvas,
	>;
	
	// TODO: Run-time checks?
	
	fn take_ownership() -> &'static mut Self {
		// JS implicitly passes ownership of the environment
		unsafe { ENV.as_mut().unwrap() }
	}
	
	fn get_state(&mut self) -> &mut Game {
		&mut self.game
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
	
	Env::init(Some(Env {
		game: Game::new(
			WebTimer::new(),
			WebCanvas::new(context),
		),
	}));
	
	let env = Env::take_ownership();
	env.game.start();
}
