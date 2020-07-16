use super::*;

use std::collections::HashMap;

use lib::{
	Init,
	Logger,
};

pub struct Chunk {
	contents: [[u16; Self::SIZE]; Self::SIZE],
}

impl Chunk {
	pub fn new(contents: [[u16; Self::SIZE]; Self::SIZE]) -> Self {
		Self {
			contents,
		}
	}
	
	pub fn get(&self, x: i32, y: i32) -> u16 {
		self.contents[y as usize][x as usize]
	}
	
	pub const SIZE: usize = 64;
	pub const I_SIZE: i32 = Self::SIZE as i32;
}

impl Init<u16, [usize; 2]> for Chunk {
	fn init<F: Fn([usize; 2]) -> u16>(elem: F) -> Self {
		Self {
			contents: <[[u16; Self::SIZE]; Self::SIZE]>::init(elem),
		}
	}
}

pub struct World {
	rules: &'static GameRules,
	settings: WorldGenParams,
	chunks: HashMap<(i32, i32), Chunk>,
}

impl World {
	pub fn new(rules: &'static GameRules, seed: u64) -> Self {
		Self {
			rules,
			settings: WorldGenParams::new(rules, seed),
			chunks: HashMap::new(),
		}
	}
	
	pub fn get(&mut self, x: i32, y: i32) -> &Block {
		let chunk_x = x.div_euclid(Chunk::I_SIZE);
		let chunk_y = y.div_euclid(Chunk::I_SIZE);
		let local_x = x.rem_euclid(Chunk::I_SIZE);
		let local_y = y.rem_euclid(Chunk::I_SIZE);
		let id =
			match self.chunks.get(&(chunk_x, chunk_y)) {
				Some(chunk) => chunk.get(local_x, local_y),
				None => {
					Logger::debug(&format!("Generating ({}, {})", chunk_x, chunk_y));
					
					let chunk = self.rules.generate_chunk(&self.settings, chunk_x, chunk_y);
					let block = chunk.get(local_x, local_y);
					self.chunks.insert((chunk_x, chunk_y), chunk);
					block
				},
			};
		let block = self.rules.block(id);
		block
	}
}
