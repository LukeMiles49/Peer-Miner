use super::Logger;

pub trait Keys<TLogger: Logger> {
	fn start(&self);
	
	fn is_pressed(&self, key: &str) -> bool;
}
