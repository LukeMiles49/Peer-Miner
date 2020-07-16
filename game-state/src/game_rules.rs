use super::*;

use std::convert::TryFrom;

use lib::{
	Colour,
	Init,
	NoiseConfig,
	NoiseDomain,
	Octaves,
	ScaleNoise,
	Seeded,
	Simplex,
	vector,
};

pub struct GameRules {
	pub blocks: Vec<Block>,
	pub depth: ScaleNoise<Octaves<Simplex, 10>, f64, f64, f64>,
}

impl GameRules {
	pub fn load() -> Self {
		let mut blocks = Vec::new();
		
		blocks.push(Block::new(Colour::try_from("#000").unwrap()));
		blocks.push(Block::new(Colour::try_from("#FFF").unwrap()));
		
		Self {
			blocks,
			depth: ScaleNoise::new(Octaves::<_, 10>::new(Simplex::new(), 2.0, 0.5), 1.0 / 4096.0, 6.0),
		}
	}
	
	pub fn block(&self, id: u16) -> &Block {
		&self.blocks[id as usize]
	}
	
	pub fn generate_chunk(&self, world: &WorldGenParams, chunk_x: i32, chunk_y: i32) -> Chunk {
		let chunk = Chunk::init(|[local_x, local_y]| -> u16 {
			let x = chunk_x * Chunk::I_SIZE + local_x as i32;
			let y = chunk_y * Chunk::I_SIZE + local_y as i32;
			let value = world.depth.noise(vector![x as f64, y as f64]);
			let value = value + (y as f64 / 32.);
			if value > 0.0 { 1 } else { 0 }
		});
		
		chunk
	}
}

pub struct WorldGenParams {
	pub depth: Seeded<ScaleNoise<Octaves<Simplex, 10>, f64, f64, f64>>,
}

impl WorldGenParams {
	pub fn new(rules: &GameRules, seed: u64) -> Self {
		Self {
			depth: rules.depth.seed(seed),
		}
	}
}
