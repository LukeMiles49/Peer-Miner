use sized_matrix::Vector;

pub struct Player {
	pos: Vector<f64, 2>,
}

impl Player {
	pub fn new(pos: Vector<f64, 2>) -> Self {
		Player {
			pos,
		}
	}
	
	pub fn pos(&self) -> Vector<f64, 2> {
		self.pos
	}
	
	pub fn tick(&mut self, k: Vector<f64, 2>) {
		self.pos += k;
	}
}
