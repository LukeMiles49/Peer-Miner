pub trait Keys {
	fn start(&self);
	
	fn is_pressed(&self, key: &str) -> bool;
}
