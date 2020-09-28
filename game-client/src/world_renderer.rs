use std::{
	cmp::*,
};

use game_interface::{
	Canvas,
	SmoothingQuality,
};

use game_state::World;

use lib::{Logger, Colour};

use sized_matrix::Vector;
use higher_order_functions::Map;
use num_traits::{Zero, PrimInt, AsPrimitive};
use noise_fn::{HashNoise, Seedable, NoiseDomain};

pub struct WorldRenderer<TCanvas: Canvas> {
	canvas: TCanvas,
	pos: Vector<i32, 2>,
	size: Vector<u32, 2>,
	noise: HashNoise,
}

impl<TCanvas: Canvas> WorldRenderer<TCanvas> {
	pub fn new(size: Vector<u32, 2>) -> Self {
		let size = size / Self::SCALE;
		
		let mut canvas = TCanvas::create(size + Vector::vector([1, 1]));
		canvas.set_smoothing_quality(SmoothingQuality::None);
		
		Self {
			canvas,
			pos: Vector::vector([i32::MIN, i32::MIN]) / 2,
			size,
			noise: HashNoise::new().seed(1234),
		}
	}
	
	pub fn draw_to(&mut self, dest: &mut TCanvas, world: &mut World, player: Vector<f64, 2>) {
		let i_size = self.size.map(|x| x as i32);
		let f_size = self.size.map(|x| x as f64);
		let f_dest = player - f_size / 2.;
		let f_canvas = f_dest.map(|x| x.floor());
		let i_canvas = f_canvas.map(|x| x as i32);
		let f_draw = ((f_dest - f_canvas) * Self::F_SCALE).map(|x| x.round()) / Self::F_SCALE;
		let i_delta = i_canvas - self.pos;
		let f_delta = i_delta.map(|x| x as f64);
		
		if i_delta != Vector::zero() {
			self.canvas.draw_self(-f_delta);
			self.pos = i_canvas;
			
			let (new_x_min, new_x_max) =
				if i_delta[0] >= 0 { (
					max(i_canvas[0] + i_size[0] + 1 - i_delta[0], i_canvas[0]),
					i_canvas[0] + i_size[0] + 1,
				) } else { (
					i_canvas[0],
					min(i_canvas[0] - i_delta[0], i_canvas[0] + i_size[0] + 1),
				) };
			
			let (new_y_min, new_y_max) =
				if i_delta[1] >= 0 { (
					max(i_canvas[1] + i_size[1] + 1 - i_delta[1], i_canvas[1]),
					i_canvas[1] + i_size[1] + 1,
				) } else { (
					i_canvas[1],
					min(i_canvas[1] - i_delta[1], i_canvas[1] + i_size[1] + 1),
				) };
			
			// Top
			if i_delta[1] >= 0 {
				for y in i_canvas[1]..new_y_min {
					for x in new_x_min..new_x_max {
						self.redraw(world, Vector::vector([x, y]));
					}
				}
			}
			
			// Side
			for y in new_y_min..new_y_max {
				for x in i_canvas[0]..(i_canvas[0] + i_size[0] + 1) {
					self.redraw(world, Vector::vector([x, y]));
				}
			}
			
			// Bottom
			if i_delta[1] < 0 {
				for y in new_y_max..(i_canvas[1] + i_size[1] + 1) {
					for x in new_x_min..new_x_max {
						self.redraw(world, Vector::vector([x, y]));
					}
				}
			}
		}
		
		dest.draw_image_segment_scaled(self.canvas.as_image(), f_draw, f_size, Vector::zero(), f_size * Self::F_SCALE);
	}
	
	pub fn redraw(&mut self, world: &mut World, pos: Vector<i32, 2>) {
		let local = pos - self.pos;
		if local[0] >= 0 && local[0] <= self.size[0] as i32 && local[1] >= 0 && local[1] <= self.size[1] as i32 {
			let block = world.get(pos);
			let mut noise = self.noise.noise(pos);
			let extra = take_value(&mut noise, block.brightness_variation);
			let colour = Colour::rgba(
				block.colour.r + extra + take_value(&mut noise, block.colour_variation),
				block.colour.g + extra + take_value(&mut noise, block.colour_variation),
				block.colour.b + extra + take_value(&mut noise, block.colour_variation),
				block.colour.a,
			);
			self.canvas.fill_rect(colour, local.map(f64::from), Vector::vector([1., 1.]));
		} else {
			Logger::warning("Drawing outside canvas");
		}
	}
	
	const SCALE: u32 = 8;
	const F_SCALE: f64 = Self::SCALE as f64;
}

fn take_value<T: PrimInt + AsPrimitive<u64>>(value: &mut u64, max: T) -> T where u64: AsPrimitive<T> {
	let max = max.as_() + 1;
	let result = *value % max;
	*value = *value / max;
	result.as_()
}
