use lib::Colour;

pub struct Block {
	pub colour: Colour,
	pub brightness_variation: u8,
	pub colour_variation: u8,
	pub solid: bool,
}

impl Block {
	pub const fn new(colour: Colour, brightness_variation: u8, colour_variation: u8, solid: bool) -> Self {
		let offset = (brightness_variation + colour_variation) / 2;
		Self {
			colour: Colour::rgba(colour.r - offset, colour.g - offset, colour.b - offset, colour.a),
			brightness_variation,
			colour_variation,
			solid,
		}
	}
	
	pub const fn fg(colour: Colour, brightness_variation: u8, colour_variation: u8) -> Self {
		Block::new(colour, brightness_variation, colour_variation, true)
	}
	
	pub const fn bg(colour: Colour, brightness_variation: u8, colour_variation: u8) -> Self {
		Block::new(colour, brightness_variation, colour_variation, false)
	}
}
