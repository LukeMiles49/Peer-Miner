use super::*;

use std::convert::TryFrom;

use lib::Colour;

use higher_order_functions::{Init, Map};
use sized_matrix::Vector;
use noise_fn::{
	Seedable,
	NoiseDomain,
	Config,
	Octaves,
	ScaleNoise,
	AddNoise,
	Gradient,
	Simplex,
	helpers::IgnoreSeed,
};

type GameNoise = AddNoise<ScaleNoise<Octaves<Simplex, 10>, f64, f64>, IgnoreSeed<u64, Gradient<f64, 2>>>;

pub struct GameRules {
	pub blocks: Vec<Block>,
	pub depth: Config<GameNoise>,
}

impl GameRules {
	pub fn load() -> Self {
		let mut blocks = Vec::new();
		
		blocks.push(Block::new(Colour::try_from("#000").unwrap()));
		blocks.push(Block::new(Colour::try_from("#FFF").unwrap()));
		
		Self {
			blocks,
			depth: AddNoise::new(
				ScaleNoise::new(
					Octaves::<_, 10>::new(
						Simplex::new(),
						2.0, 0.5,
					),
					1.0 / 4096.0, 6.0,
				),
				IgnoreSeed::new(
					Gradient::new(
						Vector::vector([0.0, 1.0 / 32.0]),
					)
				),
			),
		}
	}
	
	pub fn block(&self, id: u16) -> &Block {
		&self.blocks[id as usize]
	}
	
	pub fn generate_chunk(&self, world: &WorldGenParams, chunk: Vector<i32, 2>) -> Chunk {
		let chunk = Chunk::init(|local| -> u16 {
			let pos = chunk * Chunk::I_SIZE + local.map(|x| x as i32);
			let value = world.depth.noise(pos.map(f64::from));
			if value > 0.0 { 1 } else { 0 }
		});
		
		chunk
	}
}

pub struct WorldGenParams {
	pub depth: GameNoise,
}

impl WorldGenParams {
	pub fn new(rules: &GameRules, seed: u64) -> Self {
		Self {
			depth: rules.depth.seed(seed),
		}
	}
}
