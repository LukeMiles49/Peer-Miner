use std::{
	cell::RefCell,
	marker::PhantomData,
	mem,
	rc::{
		Rc,
		Weak,
	},
};
use wasm_bindgen::prelude::*;

use game_interface::Timer;

use super::{
	Environment,
	WebLogger,
};

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

pub struct WebTimer<Env: 'static + Environment> {
	__phantom: PhantomData<&'static mut Env>,
}

impl<Env: Environment> WebTimer<Env> {
	pub fn new() -> Self {
		WebTimer {
			__phantom: PhantomData,
		}
	}
}

impl<Env: Environment> Timer<Env::TState, WebLogger> for WebTimer<Env> {
	type TTimeout = Handle<dyn FnMut()>;
	type TInterval = Handle<dyn FnMut()>;
	type TFrame = Handle<dyn FnMut(f64)>;
	type TAnimation = Handle<dyn FnMut(f64)>;
	
	fn set_timeout<F: 'static + FnOnce(&mut Env::TState)>(&mut self, ms: u32, callback: F) -> Handle<dyn FnMut()> {
		Timeout::<Env, F>::register(ms, callback)
	}
	
	fn set_interval<F: 'static + FnMut(&mut Env::TState)>(&mut self, ms: u32, callback: F) -> Handle<dyn FnMut()> {
		Interval::<Env, F>::register(ms, callback)
	}
	
	fn set_frame<F: 'static + FnOnce(&mut Env::TState, f64)>(&mut self, callback: F) -> Handle<dyn FnMut(f64)> {
		Frame::<Env, F>::register(callback)
	}
	
	fn set_animation<F: 'static + FnMut(&mut Env::TState, f64)>(&mut self, callback: F) -> Handle<dyn FnMut(f64)> {
		Animation::<Env, F>::register(callback)
	}
}

pub struct Handle<T: ?Sized> {
	#[allow(dead_code)]
	closure: Rc<Closure<T>>,
}

impl<T: ?Sized> Handle<T> {
	pub fn new(closure: Rc<Closure<T>>) -> Self {
		Self {
			closure,
		}
	}
}


pub struct Timeout<Env: 'static + Environment, F: 'static + FnOnce(&mut Env::TState)> {
	data: Option<TimeoutData<F>>,
	
	// TODO: Not really sure why I need this?
	__phantom: PhantomData<&'static mut Env>,
}

pub struct TimeoutData<F: 'static> {
	id: f64,
	callback: F,
	#[allow(dead_code)]
	closure: Weak<Closure<dyn FnMut()>>,
}

impl<Env: 'static + Environment, F: 'static + FnOnce(&mut Env::TState)> Timeout<Env, F> {
	pub fn register(ms: u32, callback: F) -> Handle<dyn FnMut()> {
		let timeout = Rc::new(RefCell::new(Self {
			data: None,
			__phantom: PhantomData,
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
	
	fn run(&mut self) {
		// JS implicitly passes ownership of the environment
		let env = Env::take_ownership();
		let data = mem::replace(&mut self.data, None);
		(data.unwrap().callback)(env.get_state())
	}
}

impl<Env: Environment, F: 'static + FnOnce(&mut Env::TState)> Drop for Timeout<Env, F> {
	fn drop(&mut self) {
		match self.data {
			Some(TimeoutData { id, .. }) => clearTimeout(id),
			_ => (),
		}
	}
}


pub struct Interval<Env: 'static + Environment, F: 'static + FnMut(&mut Env::TState)> {
	data: Option<IntervalData<F>>,
	
	// TODO: Not really sure why I need this?
	__phantom: PhantomData<&'static mut Env>,
}

pub struct IntervalData<F: 'static> {
	id: f64,
	callback: F,
	#[allow(dead_code)]
	closure: Weak<Closure<dyn FnMut()>>,
}

impl<Env: 'static + Environment, F: 'static + FnMut(&mut Env::TState)> Interval<Env, F> {
	pub fn register(ms: u32, callback: F) -> Handle<dyn FnMut()> {
		let interval = Rc::new(RefCell::new(Self {
			data: None,
			__phantom: PhantomData,
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
	
	fn run(&mut self) {
		// JS implicitly passes ownership of the environment
		let env = Env::take_ownership();
		(self.data.as_mut().unwrap().callback)(env.get_state())
	}
}

impl<Env: 'static + Environment, F: 'static + FnMut(&mut Env::TState)> Drop for Interval<Env, F> {
	fn drop(&mut self) {
		match self.data {
			Some(IntervalData { id, .. }) => clearInterval(id),
			_ => (),
		}
	}
}


pub struct Frame<Env: 'static + Environment, F: 'static + FnOnce(&mut Env::TState, f64)> {
	data: Option<FrameData<F>>,
	
	// TODO: Not really sure why I need this?
	__phantom: PhantomData<&'static mut Env>,
}

pub struct FrameData<F: 'static> {
	id: f64,
	callback: F,
	#[allow(dead_code)]
	closure: Weak<Closure<dyn FnMut(f64)>>,
}

impl<Env: 'static + Environment, F: 'static + FnOnce(&mut Env::TState, f64)> Frame<Env, F> {
	pub fn register(callback: F) -> Handle<dyn FnMut(f64)> {
		let frame = Rc::new(RefCell::new(Self {
			data: None,
			__phantom: PhantomData,
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
	
	fn run(&mut self, time: f64) {
		// JS implicitly passes ownership of the environment
		let env = Env::take_ownership();
		let data = mem::replace(&mut self.data, None);
		(data.unwrap().callback)(env.get_state(), time)
	}
}

impl<Env: 'static + Environment, F: 'static + FnOnce(&mut Env::TState, f64)> Drop for Frame<Env, F> {
	fn drop(&mut self) {
		match self.data {
			Some(FrameData { id, .. }) => clearAnimationFrame(id),
			_ => (),
		}
	}
}


pub struct Animation<Env: 'static + Environment, F: 'static + FnMut(&mut Env::TState, f64)> {
	data: Option<AnimationData<F>>,
	
	// TODO: Not really sure why I need this?
	__phantom: PhantomData<&'static mut Env>,
}

pub struct AnimationData<F: 'static> {
	id: f64,
	callback: F,
	closure: Weak<Closure<dyn FnMut(f64)>>,
}

impl<Env: 'static + Environment, F: 'static + FnMut(&mut Env::TState, f64)> Animation<Env, F> {
	pub fn register(callback: F) -> Handle<dyn FnMut(f64)> {
		let animation = Rc::new(RefCell::new(Self {
			data: None,
			__phantom: PhantomData,
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
	
	fn run(&mut self, time: f64) {
		// JS implicitly passes ownership of the environment
		let env = Env::take_ownership();
		let data = self.data.as_mut().unwrap();
		(data.callback)(env.get_state(), time);
		data.id = requestAnimationFrame(&Weak::upgrade(&data.closure).unwrap());
	}
}

impl<Env: 'static + Environment, F: 'static + FnMut(&mut Env::TState, f64)> Drop for Animation<Env, F> {
	fn drop(&mut self) {
		match self.data {
			Some(AnimationData { id, .. }) => clearAnimationFrame(id),
			_ => (),
		}
	}
}
