use super::*;

use multiarray::*;

pub struct World {
	contents: Array2D<Block>,
}

impl World {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			contents: Array2D::new([width as usize, height as usize], Block::EMPTY),
		}
	}
	
	pub fn width(&self) -> u32 {
		self.contents.extents()[1] as u32
	}
	
	pub fn height(&self) -> u32 {
		self.contents.extents()[0] as u32
	}
	
	pub fn get(&self, x: u32, y: u32) -> Block {
		self.contents[[y as usize, x as usize]]
	}
}
