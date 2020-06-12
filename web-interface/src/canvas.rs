use web_sys::{
	CanvasRenderingContext2d,
	HtmlCanvasElement,
	HtmlImageElement,
};

use game_interface::{
	Canvas,
	Image,
};

use lib::Colour;

pub struct WebCanvas {
	canvas: CanvasRenderingContext2d,
}

impl WebCanvas {
	pub fn new(canvas: CanvasRenderingContext2d) -> Self {
		Self {
			canvas,
		}
	}
}

impl Canvas for WebCanvas {
	type TImage = WebImage;
	
	fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64, colour: Colour) {
		self.canvas.set_fill_style(&String::from(colour).into());
		self.canvas.fill_rect(x, y, w, h);
	}
	
	fn draw_image(&mut self, image: &WebImage, dx: f64, dy: f64) {
		match image {
			WebImage::ImageElement(img) => self.canvas.draw_image_with_html_image_element(img, dx, dy).unwrap(),
			WebImage::CanvasElement(img) => self.canvas.draw_image_with_html_canvas_element(img, dx, dy).unwrap(),
		};
	}
	
	fn draw_image_scaled(&mut self, image: &WebImage, dx: f64, dy: f64, dw: f64, dh: f64) {
		match image {
			WebImage::ImageElement(img) => self.canvas.draw_image_with_html_image_element_and_dw_and_dh(img, dx, dy, dw, dh).unwrap(),
			WebImage::CanvasElement(img) => self.canvas.draw_image_with_html_canvas_element_and_dw_and_dh(img, dx, dy, dw, dh).unwrap(),
		};
	}
	
	fn draw_image_segment_scaled(&mut self, image: &WebImage, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
		match image {
			WebImage::ImageElement(img) => self.canvas.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(img, sx, sy, sw, sh, dx, dy, dw, dh).unwrap(),
			WebImage::CanvasElement(img) => self.canvas.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(img, sx, sy, sw, sh, dx, dy, dw, dh).unwrap(),
		};
	}
}

pub enum WebImage {
	ImageElement(HtmlImageElement),
	CanvasElement(HtmlCanvasElement),
}

impl Image for WebImage {
	fn width(&self) -> u32 {
		match self {
			WebImage::ImageElement(img) => img.natural_width(),
			WebImage::CanvasElement(img) => img.width(),
		}
	}
	
	fn height(&self) -> u32 {
		match self {
			WebImage::ImageElement(img) => img.natural_height(),
			WebImage::CanvasElement(img) => img.height(),
		}
	}
}
