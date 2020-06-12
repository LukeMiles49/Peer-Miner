use std::convert::TryFrom;

use game_interface::{
	Canvas,
	Keys,
	Timer,
};

use game_state::{
	Block,
	Player,
	World,
};

use lib::Colour;

pub struct Game<TTimer: 'static + Timer<Self>, TCanvas: 'static + Canvas, TKeys: 'static + Keys> {
	timer: TTimer,
	canvas: TCanvas,
	keys: TKeys,
	animation: Option<TTimer::TAnimation>,
	world: Option<World>,
	player: Option<Player>,
}

impl<TTimer: 'static + Timer<Self>, TCanvas: 'static + Canvas, TKeys: 'static + Keys> Game<TTimer, TCanvas, TKeys> {
	pub fn new(timer: TTimer, canvas: TCanvas, keys: TKeys) -> Self {
		Self {
			timer,
			canvas,
			keys,
			animation: None,
			world: None,
			player: None,
		}
	}
	
	pub fn start(&mut self) {
		self.world = Some(World::new(100, 100));
		self.player = Some(Player::new(50., 50.));
		self.animation = Some(self.timer.set_animation(Self::tick));
		self.keys.start();
	}
	
	pub fn keys(&mut self) -> &mut TKeys {
		&mut self.keys
	}
	
	pub fn tick(&mut self, _time: f64) {
		let player = self.player.as_mut().unwrap();
		player.tick(
			if self.keys.is_pressed("d") { 1. } else { 0. } - if self.keys.is_pressed("a") { 1. } else { 0. },
			if self.keys.is_pressed("s") { 1. } else { 0. } - if self.keys.is_pressed("w") { 1. } else { 0. },
		);
		self.canvas.fill_rect(player.x(), player.y(), 10., 10., Colour::try_from("#777").unwrap());
	}
}
