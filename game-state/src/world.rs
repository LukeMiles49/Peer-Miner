use super::*;

use multiarray::*;

pub struct World {
	rules: &'static GameRules,
	contents: Array2D<u16>,
}

impl World {
	pub fn new(rules: &'static GameRules, width: u32, height: u32) -> Self {
		Self {
			rules,
			contents: Array2D::new([width as usize, height as usize], 0),
		}
	}
	
	pub fn width(&self) -> u32 {
		self.contents.extents()[1] as u32
	}
	
	pub fn height(&self) -> u32 {
		self.contents.extents()[0] as u32
	}
	
	pub fn get(&self, x: i32, y: i32) -> &Block {
		let id = 
			if x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32 {
				self.contents[[y as usize, x as usize]]
			} else { 1 };
		self.rules.block(id)
	}
}
