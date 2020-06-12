use std::marker::Sized;

use super::WebKeys;

pub trait Environment: Sized {
	type TState;
	
	fn take_ownership() -> &'static mut Self;
	
	fn get_state(&mut self) -> &mut Self::TState;
	
	fn get_keys(&mut self) -> &mut WebKeys<Self>;
}
