use super::*;

use std::convert::TryFrom;

use lib::Colour;

pub struct GameRules {
	pub blocks: Vec<Block>,
}

impl GameRules {
	fn new() -> Self {
		GameRules {
			blocks: Vec::new(),
		}
	}
	
	pub fn load() -> Self {
		let mut rules = Self::new();
		
		rules.blocks.push(Block::new(Colour::try_from("#FFF").unwrap()));
		rules.blocks.push(Block::new(Colour::try_from("#000").unwrap()));
		
		rules
	}
	
	pub fn block(&self, id: u16) -> &Block {
		&self.blocks[id as usize]
	}
}
