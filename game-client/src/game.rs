use super::*;

use game_interface::{
	Canvas,
	Keys,
	Timer,
	SmoothingQuality,
};

use game_state::{
	GameRules,
	Player,
	World,
};

use lib::Logger;

pub struct Game<
	TTimer: 'static + Timer<Self>,
	TCanvas: 'static + Canvas,
	TKeys: 'static + Keys,
> {
	timer: TTimer,
	canvas: TCanvas,
	keys: TKeys,
	world_renderer: WorldRenderer<TCanvas>,
	rules: GameRules,
	animation: Option<TTimer::TAnimation>,
	world: Option<World>,
	player: Option<Player>,
}

impl<
	TTimer: 'static + Timer<Self>,
	TCanvas: 'static + Canvas,
	TKeys: 'static + Keys,
> Game<TTimer, TCanvas, TKeys> {
	pub fn new(timer: TTimer, canvas: TCanvas, keys: TKeys) -> Self {
		Self {
			timer,
			keys,
			world_renderer: WorldRenderer::new(canvas.width(), canvas.height()),
			canvas,
			rules: GameRules::load(),
			animation: None,
			world: None,
			player: None,
		}
	}
	
	pub fn start(&'static mut self) {
		self.canvas.set_smoothing_quality(SmoothingQuality::None);
		self.world = Some(World::new(&self.rules, 123));
		self.player = Some(Player::new(0., 0.));
		self.animation = Some(self.timer.set_animation(Self::tick));
		self.keys.start();
		Logger::info("Started");
	}
	
	pub fn keys(&mut self) -> &mut TKeys {
		&mut self.keys
	}
	
	pub fn tick(&mut self, _time: f64) {
		let player = self.player.as_mut().unwrap();
		let world = self.world.as_mut().unwrap();
		player.tick(
			if self.keys.is_pressed("d") { 1. } else { 0. } - if self.keys.is_pressed("a") { 1. } else { 0. },
			if self.keys.is_pressed("s") { 1. } else { 0. } - if self.keys.is_pressed("w") { 1. } else { 0. },
		);
		self.world_renderer.draw_to(&mut self.canvas, world, player.x(), player.y());
	}
}
