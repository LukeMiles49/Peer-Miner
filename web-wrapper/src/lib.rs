use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{
	Element,
	HtmlCanvasElement,
};

use web_interface::{
	Environment,
	WebCanvas,
	WebKeys,
	WebTimer,
};

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = document, js_name = "getElementById")]
	fn getElementById(id: &str) -> Option<Element>;
}

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
		WebKeys<Self>,
	>;
	
	// TODO: Run-time checks?
	
	fn take_ownership() -> &'static mut Self {
		// JS implicitly passes ownership of the environment
		unsafe { ENV.as_mut().unwrap() }
	}
	
	fn get_state(&mut self) -> &mut Game {
		&mut self.game
	}
	
	fn get_keys(&mut self) -> &mut WebKeys<Self> {
		self.game.keys()
	}
}

#[wasm_bindgen(start)]
pub fn start() {
	web_interface::bind_loggers();
	#[cfg(feature = "debug")]
	web_interface::bind_panics();
	
	let canvas = getElementById("game-canvas")
		.unwrap()
		.dyn_into::<HtmlCanvasElement>()
		.unwrap();
	
	Env::init(Some(Env {
		game: Game::new(
			WebTimer::new(),
			WebCanvas::new(canvas),
			WebKeys::new(),
		),
	}));
	
	let env = Env::take_ownership();
	env.game.start();
}
