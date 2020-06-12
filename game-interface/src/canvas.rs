use lib::Colour;

pub trait Canvas {
	type TImage: Image;
	
	fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64, colour: Colour);
	
	fn draw_image(&mut self, image: &Self::TImage, dx: f64, dy: f64);
	fn draw_image_scaled(&mut self, image: &Self::TImage, dx: f64, dy: f64, dw: f64, dh: f64);
	fn draw_image_segment_scaled(&mut self, image: &Self::TImage, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64);
}

pub trait Image {
	fn width(&self) -> u32;
	fn height(&self) -> u32;
}
