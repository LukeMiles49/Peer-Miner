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
	
	pub fn draw_to(&mut self, dest: &mut TCanvas, world: &World, x: f64, y: f64) {
		let iw = self.width as i32;
		let ih = self.height as i32;
		
		let fw = self.width as f64;
		let fh = self.height as f64;
		
		let sx = x - fw / 2.;
		let sy = y - fh / 2.;
		
		let fx = sx.floor();
		let fy = sy.floor();
		
		let ix = fx as i32;
		let iy = fy as i32;
		
		let dx = ix - self.x;
		let dy = iy - self.y;
		
		self.x = ix;
		self.y = iy;
		
		if dx != 0 || dy != 0 {
			self.canvas.draw_self(-dx as f64, -dy as f64);
			
			let (new_x_min, new_x_max) =
				if dx >= 0 { (max(ix + iw + 1 - dx, ix), ix + iw + 1) }
				else { (ix, min(ix - dx, ix + iw + 1)) };
			let (new_y_min, new_y_max) =
				if dy >= 0 { (max(iy + ih + 1 - dy, iy), iy + ih + 1) }
				else { (iy, min(iy - dy, iy + ih + 1)) };
			
			if dy >= 0 {
				for y in iy..new_y_min {
					for x in new_x_min..new_x_max {
						self.redraw(world, x, y);
					}
				}
				for y in new_y_min..new_y_max {
					for x in ix..(ix + iw + 1) {
						self.redraw(world, x, y);
					}
				}
			} else {
				for y in new_y_min..new_y_max {
					for x in ix..(ix + iw + 1) {
						self.redraw(world, x, y);
					}
				}
				for y in new_y_max..(iy + ih + 1) {
					for x in new_x_min..new_x_max {
						self.redraw(world, x, y);
					}
				}
			}
		}
		
		dest.draw_image_segment_scaled(self.canvas.as_image(), sx - fx, sy - fy, fw, fh, 0., 0., fw * (Self::SCALE as f64), fh * (Self::SCALE as f64));
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
}
