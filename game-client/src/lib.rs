use std::convert::TryFrom;

use game_interface::{
	Canvas,
	Colour,
	Timer,
};

pub struct Game<TTimer: 'static + Timer<Self>, TCanvas: 'static + Canvas> {
	timer: TTimer,
	canvas: TCanvas,
	animation: Option<TTimer::TAnimation>,
}

impl<TTimer: 'static + Timer<Self>, TCanvas: 'static + Canvas> Game<TTimer, TCanvas> {
	pub fn new(timer: TTimer, canvas: TCanvas) -> Self {
		Self {
			timer,
			canvas,
			animation: None,
		}
	}
	
	pub fn start(&mut self) {
		self.animation = Some(self.timer.set_animation(Self::draw));
	}
	
	pub fn draw(&mut self, _time: f64) {
		self.canvas.fill_rect(50., 50., 100., 100., Colour::try_from("#777").unwrap());
	}
}
