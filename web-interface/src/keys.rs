use std::{
	collections::HashSet,
	marker::PhantomData,
};
use web_sys::KeyboardEvent;

use wasm_bindgen::prelude::*;

use game_interface::Keys;

use super::Environment;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = "addEventListener")]
	fn addKeyboardEventListener(event_type: &str, listener: &Closure<dyn FnMut(KeyboardEvent)>);
}

pub struct WebKeys<Env: 'static + Environment> {
	keys: HashSet<String>,
	
	on_key_down_closure: Closure<dyn FnMut(KeyboardEvent)>,
	on_key_up_closure: Closure<dyn FnMut(KeyboardEvent)>,
	
	__phantom: PhantomData<&'static mut Env>,
}

impl<Env: 'static + Environment> WebKeys<Env> {
	pub fn new() -> Self {
		Self {
			keys: HashSet::new(),
			on_key_down_closure: Closure::new(&Self::on_key_down),
			on_key_up_closure: Closure::new(&Self::on_key_up),
			__phantom: PhantomData,
		}
	}
	
	fn on_key_down(event: KeyboardEvent) {
		// JS implicitly passes ownership of the environment
		let env = Env::take_ownership();
		env.get_keys().keys.insert(event.key());
	}
	
	fn on_key_up(event: KeyboardEvent) {
		// JS implicitly passes ownership of the environment
		let env = Env::take_ownership();
		env.get_keys().keys.remove(&event.key());
	}
}

impl<Env: 'static + Environment> Keys for WebKeys<Env> {
	fn start(&self) {
		addKeyboardEventListener("keydown", &self.on_key_down_closure);
		addKeyboardEventListener("keyup", &self.on_key_up_closure);
	}
	
	fn is_pressed(&self, key: &str) -> bool {
		self.keys.contains(key)
	}
}
