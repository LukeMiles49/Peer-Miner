use lib::Colour;

pub struct Block {
	pub colour: Colour,
}

impl Block {
	pub const fn new(colour: Colour) -> Self {
		Self {
			colour,
		}
	}
}
