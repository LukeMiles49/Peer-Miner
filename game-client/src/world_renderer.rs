use std::{
	cmp::*,
	marker::PhantomData,
};

use game_interface::{
	Canvas,
	Logger,
	SmoothingQuality,
};

use game_state::World;

pub struct WorldRenderer<TCanvas: Canvas<TLogger>, TLogger: Logger> {
	canvas: TCanvas,
	x: i32,
	y: i32,
	width: u32,
	height: u32,
	
	__phantom: PhantomData<TLogger>,
}

impl<TCanvas: Canvas<TLogger>, TLogger: Logger> WorldRenderer<TCanvas, TLogger> {
	pub fn new(width: u32, height: u32) -> Self {
		let w = width / Self::SCALE;
		let h = height / Self::SCALE;
		
		let mut canvas = TCanvas::create(w + 1, h + 1);
		canvas.set_smoothing_quality(SmoothingQuality::None);
		
		Self {
			canvas,
			x: i32::MIN / 2,
			y: i32::MIN / 2,
			width: w,
			height: h,
			
			__phantom: PhantomData,
		}
	}
	
	pub fn draw_to(&mut self, dest: &mut TCanvas, world: &World, player_x: f64, player_y: f64) {
		let i_width = self.width as i32;
		let i_height = self.height as i32;
		
		let f_width = self.width as f64;
		let f_height = self.height as f64;
		
		let f_dest_x = player_x - f_width / 2.;
		let f_dest_y = player_y - f_height / 2.;
		
		let f_canvas_x = f_dest_x.floor();
		let f_canvas_y = f_dest_y.floor();
		
		let i_canvas_x = f_canvas_x as i32;
		let i_canvas_y = f_canvas_y as i32;
		
		let f_draw_x = ((f_dest_x - f_canvas_x) * Self::F_SCALE).round() / Self::F_SCALE;
		let f_draw_y = ((f_dest_y - f_canvas_y) * Self::F_SCALE).round() / Self::F_SCALE;
		
		let f_delta_x = i_canvas_x - self.x;
		let f_delta_y = i_canvas_y - self.y;
		
		if f_delta_x != 0 || f_delta_y != 0 {
			self.canvas.draw_self(-f_delta_x as f64, -f_delta_y as f64);
			self.x = i_canvas_x;
			self.y = i_canvas_y;
			
			let (new_x_min, new_x_max) =
				if f_delta_x >= 0 { (
					max(i_canvas_x + i_width + 1 - f_delta_x, i_canvas_x),
					i_canvas_x + i_width + 1,
				) } else { (
					i_canvas_x,
					min(i_canvas_x - f_delta_x, i_canvas_x + i_width + 1),
				) };
			
			let (new_y_min, new_y_max) =
				if f_delta_y >= 0 { (
					max(i_canvas_y + i_height + 1 - f_delta_y, i_canvas_y),
					i_canvas_y + i_height + 1,
				) } else { (
					i_canvas_y,
					min(i_canvas_y - f_delta_y, i_canvas_y + i_height + 1),
				) };
			
			// Top
			if f_delta_y >= 0 {
				for y in i_canvas_y..new_y_min {
					for x in new_x_min..new_x_max {
						self.redraw(world, x, y);
					}
				}
			}
			
			// Side
			for y in new_y_min..new_y_max {
				for x in i_canvas_x..(i_canvas_x + i_width + 1) {
					self.redraw(world, x, y);
				}
			}
			
			// Bottom
			if f_delta_y < 0 {
				for y in new_y_max..(i_canvas_y + i_height + 1) {
					for x in new_x_min..new_x_max {
						self.redraw(world, x, y);
					}
				}
			}
		}
		
		dest.draw_image_segment_scaled(
			self.canvas.as_image(),
			f_draw_x, f_draw_y, f_width, f_height,
			0., 0., f_width * Self::F_SCALE, f_height * Self::F_SCALE,
		);
	}
	
	pub fn redraw(&mut self, world: &World, x: i32, y: i32) {
		let local_x = x - self.x;
		let local_y = y - self.y;
		if local_x >= 0 && local_x <= self.width as i32 && local_y >= 0 && local_y <= self.height as i32 {
			let block = world.get(x, y);
			self.canvas.fill_rect(block.colour, local_x as f64, local_y as f64, 1., 1.);
		} else {
			TLogger::warning("Drawing outside canvas");
		}
	}
	
	const SCALE: u32 = 4;
	const F_SCALE: f64 = Self::SCALE as f64;
}
