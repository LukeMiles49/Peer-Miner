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

use sized_matrix::Vector;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = document, js_name = "createElement")]
	fn createCanvasElement(element_type: &str) -> HtmlCanvasElement;
}

// TODO: Fast WASM canvas using ImagaData pointing to WASM memory?

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
	fn size(&self) -> Vector<u32, 2> {
		Vector::vector([self.canvas.width(), self.canvas.height()])
	}
}

impl Canvas for WebCanvas {
	type TImage<'a> = WebImage<'a>;
	
	fn create(size: Vector<u32, 2>) -> Self {
		let canvas = createCanvasElement("canvas");
		
		canvas.set_width(size[0]);
		canvas.set_height(size[1]);
		
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
	
	fn fill_rect(&mut self, colour: Colour, pos: Vector<f64, 2>, size: Vector<f64, 2>) {
		self.context.set_fill_style(&String::from(colour).into());
		self.context.fill_rect(pos[0], pos[1], size[0], size[1]);
	}
	
	fn draw_image(&mut self, image: WebImage, pos: Vector<f64, 2>) {
		match image {
			WebImage::ImageElement(img) => self.context.draw_image_with_html_image_element(img, pos[0], pos[1]).unwrap(),
			WebImage::CanvasElement(img) => self.context.draw_image_with_html_canvas_element(img, pos[0], pos[1]).unwrap(),
		};
	}
	
	fn draw_image_scaled(&mut self, image: WebImage, pos: Vector<f64, 2>, size: Vector<f64, 2>) {
		match image {
			WebImage::ImageElement(img) => self.context.draw_image_with_html_image_element_and_dw_and_dh(img, pos[0], pos[1], size[0], size[1]).unwrap(),
			WebImage::CanvasElement(img) => self.context.draw_image_with_html_canvas_element_and_dw_and_dh(img, pos[0], pos[1], size[0], size[1]).unwrap(),
		};
	}
	
	fn draw_image_segment_scaled(&mut self, image: WebImage, source_pos: Vector<f64, 2>, source_size: Vector<f64, 2>, dest_pos: Vector<f64, 2>, dest_size: Vector<f64, 2>) {
		match image {
			WebImage::ImageElement(img) => self.context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(img, source_pos[0], source_pos[1], source_size[0], source_size[1], dest_pos[0], dest_pos[1], dest_size[0], dest_size[1]).unwrap(),
			WebImage::CanvasElement(img) => self.context.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(img, source_pos[0], source_pos[1], source_size[0], source_size[1], dest_pos[0], dest_pos[1], dest_size[0], dest_size[1]).unwrap(),
		};
	}
	
	fn draw_self(&mut self, pos: Vector<f64, 2>) {
		self.context.draw_image_with_html_canvas_element(&self.canvas, pos[0], pos[1]).unwrap()
	}
	
	fn draw_self_scaled(&mut self, pos: Vector<f64, 2>, size: Vector<f64, 2>) {
		self.context.draw_image_with_html_canvas_element_and_dw_and_dh(&self.canvas, pos[0], pos[1], size[0], size[1]).unwrap()
	}
	
	fn draw_self_segment_scaled(&mut self, source_pos: Vector<f64, 2>, source_size: Vector<f64, 2>, dest_pos: Vector<f64, 2>, dest_size: Vector<f64, 2>) {
		self.context.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self.canvas, source_pos[0], source_pos[1], source_size[0], source_size[1], dest_pos[0], dest_pos[1], dest_size[0], dest_size[1]).unwrap()
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
	fn size(&self) -> Vector<u32, 2> {
		match self {
			WebImage::ImageElement(img) => Vector::vector([img.natural_width(), img.natural_height()]),
			WebImage::CanvasElement(img) => Vector::vector([img.width(), img.height()]),
		}
	}
}
