pub trait Environment {
	type TState;
	
	fn take_ownership() -> &'static mut Self;
	
	fn get_state(&mut self) -> &mut Self::TState;
}
