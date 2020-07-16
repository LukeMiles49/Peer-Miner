use lib::Colour;

pub trait Image {
	fn width(&self) -> u32;
	fn height(&self) -> u32;
}

pub trait Canvas: Image {
	type TImage<'a>: Image;
	
	fn create(width: u32, height: u32) -> Self;
	
	fn set_smoothing_quality(&mut self, quality: SmoothingQuality);
	
	fn fill_rect(&mut self, colour: Colour, x: f64, y: f64, w: f64, h: f64);
	
	fn draw_image(&mut self, image: Self::TImage<'_>, dx: f64, dy: f64);
	fn draw_image_scaled(&mut self, image: Self::TImage<'_>, dx: f64, dy: f64, dw: f64, dh: f64);
	fn draw_image_segment_scaled(&mut self, image: Self::TImage<'_>, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64);
	
	fn draw_self(&mut self, dx: f64, dy: f64);
	fn draw_self_scaled(&mut self, dx: f64, dy: f64, dw: f64, dh: f64);
	fn draw_self_segment_scaled(&mut self, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64);
	
	fn as_image(&self) -> Self::TImage<'_>;
}

pub enum SmoothingQuality {
	None,
	Low,
	Medium,
	High,
}
