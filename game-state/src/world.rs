use super::*;

use std::collections::HashMap;

use lib::Logger;

use higher_order_functions::Init;

use sized_matrix::Vector;

pub struct Chunk {
	contents: [[u16; Self::SIZE]; Self::SIZE],
}

impl Chunk {
	pub fn new(contents: [[u16; Self::SIZE]; Self::SIZE]) -> Self {
		Self {
			contents,
		}
	}
	
	pub fn get(&self, pos: Vector<i32, 2>) -> u16 {
		self.contents[pos[1] as usize][pos[0] as usize]
	}
	
	pub const SIZE: usize = 64;
	pub const I_SIZE: i32 = Self::SIZE as i32;
}

impl Init<u16, Vector<usize, 2>> for Chunk {
	fn init_with<F: FnMut(Vector<usize, 2>) -> u16>(_: (), mut elem: F) -> Self {
		Self {
			contents: <[_; Self::SIZE]>::init(|y| <[_; Self::SIZE]>::init(|x| elem(Vector::vector([x, y])))),
		}
	}
}

pub struct World {
	rules: &'static GameRules,
	settings: WorldGenParams,
	chunks: HashMap<Vector<i32, 2>, Chunk>,
}

impl World {
	pub fn new(rules: &'static GameRules, seed: u64) -> Self {
		Self {
			rules,
			settings: WorldGenParams::new(rules, seed),
			chunks: HashMap::new(),
		}
	}
	
	pub fn get(&mut self, pos: Vector<i32, 2>) -> &Block {
		let chunk_pos = Vector::vector([pos[0].div_euclid(Chunk::I_SIZE), pos[1].div_euclid(Chunk::I_SIZE)]);
		let local_pos = Vector::vector([pos[0].rem_euclid(Chunk::I_SIZE), pos[1].rem_euclid(Chunk::I_SIZE)]);
		let id =
			match self.chunks.get(&chunk_pos) {
				Some(chunk) => chunk.get(local_pos),
				None => {
					Logger::debug(&format!("Generating ({}, {})", chunk_pos[0], chunk_pos[1]));
					
					let chunk = self.rules.generate_chunk(&self.settings, chunk_pos);
					let block = chunk.get(local_pos);
					self.chunks.insert(chunk_pos, chunk);
					block
				},
			};
		let block = self.rules.block(id);
		block
	}
}
