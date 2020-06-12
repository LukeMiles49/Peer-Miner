pub struct Player {
	x: f64,
	y: f64,
}

impl Player {
	pub fn new(x: f64, y: f64) -> Self {
		Player {
			x,
			y,
		}
	}
	
	pub fn x(&self) -> f64 {
		self.x
	}
	
	pub fn y(&self) -> f64 {
		self.y
	}
	
	pub fn tick(&mut self, kx: f64, ky: f64) {
		self.x += kx;
		self.y += ky;
	}
}
