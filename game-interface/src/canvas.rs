use lib::Colour;
use sized_matrix::Vector;

pub trait Image {
	fn size(&self) -> Vector<u32, 2>;
}

pub trait Canvas: Image {
	type TImage<'a>: Image;
	
	fn create(size: Vector<u32, 2>) -> Self;
	
	fn set_smoothing_quality(&mut self, quality: SmoothingQuality);
	
	fn fill_rect(&mut self, colour: Colour, pos: Vector<f64, 2>, size: Vector<f64, 2>);
	
	fn draw_image(&mut self, image: Self::TImage<'_>, pos: Vector<f64, 2>);
	fn draw_image_scaled(&mut self, image: Self::TImage<'_>, pos: Vector<f64, 2>, size: Vector<f64, 2>);
	fn draw_image_segment_scaled(&mut self, image: Self::TImage<'_>, source_pos: Vector<f64, 2>, source_size: Vector<f64, 2>, dest_pos: Vector<f64, 2>, dest_size: Vector<f64, 2>);
	
	fn draw_self(&mut self, pos: Vector<f64, 2>);
	fn draw_self_scaled(&mut self, pos: Vector<f64, 2>, size: Vector<f64, 2>);
	fn draw_self_segment_scaled(&mut self, source_pos: Vector<f64, 2>, source_size: Vector<f64, 2>, dest_pos: Vector<f64, 2>, dest_size: Vector<f64, 2>);
	
	fn as_image(&self) -> Self::TImage<'_>;
}

pub enum SmoothingQuality {
	None,
	Low,
	Medium,
	High,
}
