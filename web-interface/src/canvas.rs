use super::WebLogger;

use web_sys::{
	CanvasRenderingContext2d,
	HtmlCanvasElement,
	HtmlImageElement,
};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use game_interface::{
	Canvas,
	Image,
	SmoothingQuality,
};

use lib::Colour;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = document, js_name = "createElement")]
	fn createCanvasElement(element_type: &str) -> HtmlCanvasElement;
}

pub struct WebCanvas {
	canvas: HtmlCanvasElement,
	context: CanvasRenderingContext2d,
}

impl WebCanvas {
	pub fn new(canvas: HtmlCanvasElement) -> Self {
		Self {
			context: canvas
				.get_context("2d")
				.unwrap()
				.unwrap()
				.dyn_into::<web_sys::CanvasRenderingContext2d>()
				.unwrap(),
			canvas,
		}
	}
}

impl Image for WebCanvas {
	fn width(&self) -> u32 {
		self.canvas.width()
	}
	
	fn height(&self) -> u32 {
		self.canvas.height()
	}
}

impl Canvas<WebLogger> for WebCanvas {
	type TImage<'a> = WebImage<'a>;
	
	fn create(width: u32, height: u32) -> Self {
		let canvas = createCanvasElement("canvas");
		
		canvas.set_width(width);
		canvas.set_height(height);
		
		Self::new(canvas)
	}
	
	fn set_smoothing_quality(&mut self, quality: SmoothingQuality) {
		match quality {
			SmoothingQuality::None => self.context.set_image_smoothing_enabled(false),
			_ => {
				self.context.set_image_smoothing_enabled(true);
				// TODO: Set quality when supported
			},
		}
	}
	
	fn fill_rect(&mut self, colour: Colour, x: f64, y: f64, w: f64, h: f64) {
		self.context.set_fill_style(&String::from(colour).into());
		self.context.fill_rect(x, y, w, h);
	}
	
	fn draw_image(&mut self, image: WebImage, dx: f64, dy: f64) {
		match image {
			WebImage::ImageElement(img) => self.context.draw_image_with_html_image_element(img, dx, dy).unwrap(),
			WebImage::CanvasElement(img) => self.context.draw_image_with_html_canvas_element(img, dx, dy).unwrap(),
		};
	}
	
	fn draw_image_scaled(&mut self, image: WebImage, dx: f64, dy: f64, dw: f64, dh: f64) {
		match image {
			WebImage::ImageElement(img) => self.context.draw_image_with_html_image_element_and_dw_and_dh(img, dx, dy, dw, dh).unwrap(),
			WebImage::CanvasElement(img) => self.context.draw_image_with_html_canvas_element_and_dw_and_dh(img, dx, dy, dw, dh).unwrap(),
		};
	}
	
	fn draw_image_segment_scaled(&mut self, image: WebImage, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
		match image {
			WebImage::ImageElement(img) => self.context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(img, sx, sy, sw, sh, dx, dy, dw, dh).unwrap(),
			WebImage::CanvasElement(img) => self.context.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(img, sx, sy, sw, sh, dx, dy, dw, dh).unwrap(),
		};
	}
	
	fn draw_self(&mut self, dx: f64, dy: f64) {
		self.context.draw_image_with_html_canvas_element(&self.canvas, dx, dy).unwrap()
	}
	
	fn draw_self_scaled(&mut self, dx: f64, dy: f64, dw: f64, dh: f64) {
		self.context.draw_image_with_html_canvas_element_and_dw_and_dh(&self.canvas, dx, dy, dw, dh).unwrap()
	}
	
	fn draw_self_segment_scaled(&mut self, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
		self.context.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self.canvas, sx, sy, sw, sh, dx, dy, dw, dh).unwrap()
	}
	
	
	fn as_image(&self) -> WebImage {
		WebImage::CanvasElement(&self.canvas)
	}
}

pub enum WebImage<'a> {
	ImageElement(&'a HtmlImageElement),
	CanvasElement(&'a HtmlCanvasElement),
}

impl Image for WebImage<'_> {
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
