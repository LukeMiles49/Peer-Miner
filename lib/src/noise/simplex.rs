use super::{
	Noise,
	NoiseDomain,
	PermSeeded,
	super::Vector,
};

use lazy_static::lazy_static;


// http://webstaff.itn.liu.se/%7Estegu/simplexnoise/

#[derive(Copy, Clone)]
pub struct Simplex;

impl Simplex {
	pub fn new() -> Self {
		Self { }
	}
}

const GRAD3: [[i8; 3]; 12] = [
	[1, 1, 0], [-1, 1, 0], [1, -1, 0], [-1, -1, 0],
	[1, 0, 1], [-1, 0, 1], [1, 0, -1], [-1, 0, -1],
	[0, 1, 1], [0, -1, 1], [0, 1, -1], [0, -1, -1],
];

/*
const GRAD4: [[i8; 4]; 32] = [
	[0, 1, 1, 1], [0, 1, 1, -1], [0, 1, -1, 1], [0, 1, -1, -1],
	[0, -1, 1, 1], [0, -1, 1, -1], [0, -1, -1, 1], [0, -1, -1, -1],
	[1, 0, 1, 1], [1, 0, 1, -1], [1, 0, -1, 1], [1, 0, -1, -1],
	[-1, 0, 1, 1], [-1, 0, 1, -1], [-1, 0, -1, 1], [-1, 0, -1, -1],
	[1, 1, 0, 1], [1, 1, 0, -1], [1, -1, 0, 1], [1, -1, 0, -1],
	[-1, 1, 0, 1], [-1, 1, 0, -1], [-1, -1, 0, 1], [-1, -1, 0, -1],
	[1, 1, 1, 0], [1, 1, -1, 0], [1, -1, 1, 0], [1, -1, -1, 0],
	[-1, 1, 1, 0], [-1, 1, -1, 0], [-1, -1, 1, 0], [-1, -1, -1, 0],
];
*/

// FIXME: Change to const when sqrt is supported

fn F(n: u32) -> f64 {
	let n = n as f64;
	((n + 1.).sqrt() - 1.) / n
}

fn G(n: u32) -> f64 {
	let n = n as f64;
	(n + 1. - (n + 1.).sqrt()) / (n * (n + 1.))
}

lazy_static! {
	static ref F2: f64 = F(2);
	static ref G2: f64 = G(2);
	static ref F3: f64 = F(3);
	static ref G3: f64 = G(3);
	static ref F4: f64 = F(4);
	static ref G4: f64 = G(4);
}

impl Noise for PermSeeded<Simplex> { }

impl NoiseDomain<Vector<f64, 2>, f64> for PermSeeded<Simplex> {
	fn noise(&self, point: Vector<f64, 2>) -> f64 {
		let xin = point[0];
		let yin = point[1];
		let s = (xin + yin) * *F2;
		let i = (xin + s).floor();
		let j = (yin + s).floor();
		let t = (i + j) * *G2;
		let X0 = i - t;
		let Y0 = j - t;
		let x0 = xin - X0;
		let y0 = yin - Y0;
		let (i1, j1) =
			if x0 > y0 { (1, 0) }
			else { (0, 1) };
		let x1 = x0 - i1 as f64 + *G2;
		let y1 = y0 - j1 as f64 + *G2;
		let x2 = x0 - 1. + 2. * *G2;
		let y2 = y0 - 1. + 2. * *G2;
		let ii = (i as i64).rem_euclid(256) as u8;
		let jj = (j as i64).rem_euclid(256) as u8;
		let get_perm = |x: u8, y: u8| -> u8 {
			self.perm_table.0[(ii.wrapping_add(x).wrapping_add(self.perm_table.0[jj.wrapping_add(y) as usize])) as usize] % 12
		};
		let gi0 = get_perm(0, 0);
		let gi1 = get_perm(i1, j1);
		let gi2 = get_perm(1, 1);
		let t0 = 0.5 - x0 * x0 - y0 * y0;
		let grad = |g: u8, x: f64, y: f64| {
			let g = GRAD3[g as usize];
			g[0] as f64 * x + g[1] as f64 * y
		};
		let n0 =
			if t0 < 0. { 0.0 }
			else { t0.powi(4) * grad(gi0, x0, y0) };
		let t1 = 0.5 - x1 * x1 - y1 * y1;
		let n1 =
			if t1 < 0. { 0.0 }
			else { t1.powi(4) * grad(gi1, x1, y1) };
		let t2 = 0.5 - x2 * x2 - y2 * y2;
		let n2 =
			if t2 < 0. { 0.0 }
			else { t2.powi(4) * grad(gi2, x2, y2) };
		return 70.0 * (n0 + n1 + n2)
	}
}
