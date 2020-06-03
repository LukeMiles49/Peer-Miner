use std::{
	cell::{
		RefCell,
	},
	mem,
	rc::{
		Rc,
		Weak,
	},
};
use wasm_bindgen::prelude::*;

use game_client::Timer;

use crate::Environment;

use super::Game;

#[wasm_bindgen]
extern "C" {
	// TODO: Switch to FnOnce once it is supported
	fn setTimeout(callback: &Closure<dyn FnMut()>, millis: u32) -> f64;
	fn setInterval(callback: &Closure<dyn FnMut()>, millis: u32) -> f64;
	fn requestAnimationFrame(callback: &Closure<dyn FnMut(f64)>) -> f64;
	fn clearTimeout(id: f64);
	fn clearInterval(id: f64);
	fn clearAnimationFrame(id: f64);
}

pub struct WebTimer { }

impl WebTimer {
	pub fn new() -> Self {
		WebTimer { }
	}
}

impl Timer<Game> for WebTimer {
	type TTimeout = Handle<dyn FnMut()>;
	type TInterval = Handle<dyn FnMut()>;
	type TFrame = Handle<dyn FnMut(f64)>;
	type TAnimation = Handle<dyn FnMut(f64)>;
	
	fn set_timeout<F: 'static + FnOnce(&mut Game)>(&mut self, ms: u32, callback: F) -> Handle<dyn FnMut()> {
		Timeout::register(ms, callback)
	}
	
	fn set_interval<F: 'static + FnMut(&mut Game)>(&mut self, ms: u32, callback: F) -> Handle<dyn FnMut()> {
		Interval::register(ms, callback)
	}
	
	fn set_frame<F: 'static + FnOnce(&mut Game, f64)>(&mut self, callback: F) -> Handle<dyn FnMut(f64)> {
		Frame::register(callback)
	}
	
	fn set_animation<F: 'static + FnMut(&mut Game, f64)>(&mut self, callback: F) -> Handle<dyn FnMut(f64)> {
		Animation::register(callback)
	}
}

pub struct Handle<T: ?Sized> {
	closure: Rc<Closure<T>>,
}

impl<T: ?Sized> Handle<T> {
	pub fn new(closure: Rc<Closure<T>>) -> Self {
		Self {
			closure,
		}
	}
}


pub struct Timeout<F: 'static + FnOnce(&mut Game)> {
	data: Option<TimeoutData<F>>,
}

pub struct TimeoutData<F: 'static> {
	id: f64,
	callback: F,
	closure: Weak<Closure<dyn FnMut()>>,
}

impl<F: 'static + FnOnce(&mut Game)> Timeout<F> {
	pub fn register(ms: u32, callback: F) -> Handle<dyn FnMut()> {
		let timeout = Rc::new(RefCell::new(Self {
			data: None,
		}));
		
		let closure = {
			let timeout = Rc::clone(&timeout);
			Rc::new(Closure::new(move || timeout.borrow_mut().run()))
		};
		let id = setTimeout(&closure, ms);
		timeout.borrow_mut().data = Some(TimeoutData {
			closure: Rc::downgrade(&closure),
			id,
			callback,
		});
		
		Handle::new(closure)
	}
}

impl<F: 'static + FnOnce(&mut Game)> Drop for Timeout<F> {
	fn drop(&mut self) {
		match self.data {
			Some(TimeoutData { id, .. }) => clearTimeout(id),
			_ => (),
		}
	}
}

impl<F: 'static + FnOnce(&mut Game)> Timeout<F> {
	pub fn run(&mut self) {
		// JS implicitly passes ownership of the environment
		let env = Environment::take_ownership();
		let data = mem::replace(&mut self.data, None);
		(data.unwrap().callback)(&mut env.game)
	}
}


pub struct Interval<F: 'static + FnMut(&mut Game)> {
	data: Option<IntervalData<F>>,
}

pub struct IntervalData<F: 'static> {
	id: f64,
	callback: F,
	closure: Weak<Closure<dyn FnMut()>>,
}

impl<F: 'static + FnMut(&mut Game)> Interval<F> {
	pub fn register(ms: u32, callback: F) -> Handle<dyn FnMut()> {
		let interval = Rc::new(RefCell::new(Self {
			data: None,
		}));
		
		let closure = {
			let interval = Rc::clone(&interval);
			Rc::new(Closure::new(move || interval.borrow_mut().run()))
		};
		let id = setInterval(&closure, ms);
		interval.borrow_mut().data = Some(IntervalData {
			closure: Rc::downgrade(&closure),
			id,
			callback,
		});
		
		Handle::new(closure)
	}
}

impl<F: 'static + FnMut(&mut Game)> Drop for Interval<F> {
	fn drop(&mut self) {
		match self.data {
			Some(IntervalData { id, .. }) => clearInterval(id),
			_ => (),
		}
	}
}

impl<F: 'static + FnMut(&mut Game)> Interval<F> {
	pub fn run(&mut self) {
		// JS implicitly passes ownership of the environment
		let env = Environment::take_ownership();
		(self.data.as_mut().unwrap().callback)(&mut env.game)
	}
}


pub struct Frame<F: 'static + FnOnce(&mut Game, f64)> {
	data: Option<FrameData<F>>,
}

pub struct FrameData<F: 'static> {
	id: f64,
	callback: F,
	closure: Weak<Closure<dyn FnMut(f64)>>,
}

impl<F: 'static + FnOnce(&mut Game, f64)> Frame<F> {
	pub fn register(callback: F) -> Handle<dyn FnMut(f64)> {
		let frame = Rc::new(RefCell::new(Self {
			data: None,
		}));
		
		let closure = {
			let frame = Rc::clone(&frame);
			Rc::new(Closure::new(move |time| frame.borrow_mut().run(time)))
		};
		let id = requestAnimationFrame(&closure);
		frame.borrow_mut().data = Some(FrameData {
			closure: Rc::downgrade(&closure),
			id,
			callback,
		});
		
		Handle::new(closure)
	}
}

impl<F: 'static + FnOnce(&mut Game, f64)> Drop for Frame<F> {
	fn drop(&mut self) {
		match self.data {
			Some(FrameData { id, .. }) => clearAnimationFrame(id),
			_ => (),
		}
	}
}

impl<F: 'static + FnOnce(&mut Game, f64)> Frame<F> {
	pub fn run(&mut self, time: f64) {
		// JS implicitly passes ownership of the environment
		let env = Environment::take_ownership();
		let data = mem::replace(&mut self.data, None);
		(data.unwrap().callback)(&mut env.game, time)
	}
}


pub struct Animation<F: 'static + FnMut(&mut Game, f64)> {
	data: Option<AnimationData<F>>,
}

pub struct AnimationData<F: 'static> {
	id: f64,
	callback: F,
	closure: Weak<Closure<dyn FnMut(f64)>>,
}

impl<F: 'static + FnMut(&mut Game, f64)> Animation<F> {
	pub fn register(callback: F) -> Handle<dyn FnMut(f64)> {
		let animation = Rc::new(RefCell::new(Self {
			data: None,
		}));
		
		let closure = {
			let animation = Rc::clone(&animation);
			Rc::new(Closure::new(move |time| animation.borrow_mut().run(time)))
		};
		let id = requestAnimationFrame(&closure);
		animation.borrow_mut().data = Some(AnimationData {
			closure: Rc::downgrade(&closure),
			id,
			callback,
		});
		
		Handle::new(closure)
	}
}

impl<F: 'static + FnMut(&mut Game, f64)> Drop for Animation<F> {
	fn drop(&mut self) {
		match self.data {
			Some(AnimationData { id, .. }) => clearAnimationFrame(id),
			_ => (),
		}
	}
}

impl<F: 'static + FnMut(&mut Game, f64)> Animation<F> {
	pub fn run(&mut self, time: f64) {
		// JS implicitly passes ownership of the environment
		let env = Environment::take_ownership();
		let data = self.data.as_mut().unwrap();
		(data.callback)(&mut env.game, time);
		data.id = requestAnimationFrame(&Weak::upgrade(&data.closure).unwrap());
	}
}
