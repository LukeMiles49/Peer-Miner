#[derive(Copy, Clone)]
pub struct Block {
	id: u16,
}

impl Block {
	pub const fn new(id: u16) -> Self {
		Self {
			id,
		}
	}
	
	pub const EMPTY: Self = Block::new(0);
}
