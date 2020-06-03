mod canvas;
pub use canvas::*;

mod timer;
pub use timer::*;

pub type Game = game_client::Game<
	WebTimer,
	WebCanvas,
>;
