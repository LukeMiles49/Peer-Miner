use super::Logger;

pub trait Timer<TState, TLogger: Logger> {
	// TODO: Make generic to avoid box when generic associated types are stable
	
	type TTimeout;
	type TInterval;
	type TFrame;
	type TAnimation;
	
	fn set_timeout<F: 'static + FnOnce(&mut TState)>(&mut self, ms: u32, callback: F) -> Self::TTimeout;
	fn set_interval<F: 'static + FnMut(&mut TState)>(&mut self, ms: u32, callback: F) -> Self::TInterval;
	fn set_frame<F: 'static + FnOnce(&mut TState, f64)>(&mut self, callback: F) -> Self::TFrame;
	fn set_animation<F: 'static + FnMut(&mut TState, f64)>(&mut self, callback: F) -> Self::TAnimation;
}
