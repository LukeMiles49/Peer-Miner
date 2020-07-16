use std::sync::RwLock;

pub struct Atomic<T: Copy> {
	value: RwLock<T>,
}

impl<T: Copy> Atomic<T> {
	pub fn new(value: T) -> Self {
		Self {
			value: RwLock::new(value),
		}
	}
	
	pub fn get(&self) -> T {
		*self.value.read().unwrap()
	}
	
	pub fn set(&self, value: T) {
		*self.value.write().unwrap() = value
	}
}
