use super::{
	Noise,
	NoiseDomain,
	PermSeeded,
	PermTable,
	super::Vector,
};

use lazy_static::lazy_static;


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

// FIXME: Change to const when sqrt is supported

fn f(n: u32) -> f64 {
	let n = n as f64;
	((n + 1.).sqrt() - 1.) / n
}

fn g(n: u32) -> f64 {
	let n = n as f64;
	(n + 1. - (n + 1.).sqrt()) / (n * (n + 1.))
}

lazy_static! {
	static ref F2: f64 = f(2);
	static ref G2: f64 = g(2);
	static ref F3: f64 = f(3);
	static ref G3: f64 = g(3);
	static ref F4: f64 = f(4);
	static ref G4: f64 = g(4);
}

impl Noise for PermSeeded<Simplex> { }

fn simplex_factor_2(perm_table: &PermTable, base_x: u8, base_y: u8, rel_x: f64, rel_y: f64) -> f64 {
	let t = 0.5 - rel_x.powi(2) - rel_y.powi(2);
	if t < 0. { 0.0 }
	else {
		let p = perm_table.0[base_x.wrapping_add(
			perm_table.0[base_y as usize]
		) as usize] % 12;
		let g = GRAD3[p as usize];
		t.powi(4) * (g[0] as f64 * rel_x + g[1] as f64 * rel_y)
	}
}

impl NoiseDomain<Vector<f64, 2>, f64> for PermSeeded<Simplex> {
	fn noise(&self, point: Vector<f64, 2>) -> f64 {
		let xin = point[0];
		let yin = point[1];
		
		let s = (xin + yin) * *F2;
		let base_x = (xin + s).floor();
		let base_y = (yin + s).floor();
		
		let t = (base_x + base_y) * *G2;
		let rel_x = xin - base_x + t;
		let rel_y = yin - base_y + t;
		
		let base_x = (base_x as i64).rem_euclid(256) as u8;
		let base_y = (base_y as i64).rem_euclid(256) as u8;
		
		return 70.0 * (
			simplex_factor_2(&self.perm_table,
				base_x, base_y,
				rel_x, rel_y)
			+
			if rel_x >= rel_y {
				simplex_factor_2(&self.perm_table,
					base_x.wrapping_add(1), base_y,
					rel_x - 1. + *G2, rel_y + *G2) }
			else {
				simplex_factor_2(&self.perm_table,
					base_x, base_y.wrapping_add(1),
					rel_x + *G2, rel_y - 1. + *G2) }
			+
			simplex_factor_2(&self.perm_table,
				base_x.wrapping_add(1), base_y.wrapping_add(1),
				rel_x - 1. + 2. * *G2, rel_y - 1. + 2. * *G2))
	}
}

fn simplex_factor_3(perm_table: &PermTable, base_x: u8, base_y: u8, base_z: u8, rel_x: f64, rel_y: f64, rel_z: f64) -> f64 {
	let t = 0.6 - rel_x.powi(2) - rel_y.powi(2) - rel_z.powi(2);
	if t < 0. { 0.0 }
	else {
		let p = perm_table.0[base_x.wrapping_add(
			perm_table.0[base_y.wrapping_add(
				perm_table.0[base_z as usize]
			) as usize]
		) as usize] % 12;
		let g = GRAD3[p as usize];
		t.powi(4) * (g[0] as f64 * rel_x + g[1] as f64 * rel_y + g[2] as f64 * rel_z)
	}
}

impl NoiseDomain<Vector<f64, 3>, f64> for PermSeeded<Simplex> {
	fn noise(&self, point: Vector<f64, 3>) -> f64 {
		let xin = point[0];
		let yin = point[1];
		let zin = point[2];
		
		let s = (xin + yin + zin) * *F3;
		let base_x = (xin + s).floor();
		let base_y = (yin + s).floor();
		let base_z = (zin + s).floor();
		
		let t = (base_x + base_y + base_z) * *G3;
		let rel_x = xin - base_x + t;
		let rel_y = yin - base_y + t;
		let rel_z = zin - base_z + t;
		
		let base_x = (base_x as i64).rem_euclid(256) as u8;
		let base_y = (base_y as i64).rem_euclid(256) as u8;
		let base_z = (base_z as i64).rem_euclid(256) as u8;
		
		return 32.0 * (
			simplex_factor_3(&self.perm_table,
				base_x, base_y, base_z,
				rel_x, rel_y, rel_z)
			+
			if rel_x >= rel_y && rel_x >= rel_z {
				simplex_factor_3(&self.perm_table,
					base_x.wrapping_add(1), base_y, base_z,
					rel_x - 1. + *G3, rel_y + *G3, rel_z + *G3) }
			else if rel_y >= rel_x && rel_y >= rel_z {
				simplex_factor_3(&self.perm_table,
					base_x, base_y.wrapping_add(1), base_z,
					rel_x + *G3, rel_y - 1. + *G3, rel_z + *G3) }
			else {
				simplex_factor_3(&self.perm_table,
					base_x, base_y, base_z.wrapping_add(1),
					rel_x + *G3, rel_y + *G3, rel_z - 1. + *G3) }
			+
			if rel_x >= rel_z && rel_y >= rel_z {
				simplex_factor_3(&self.perm_table,
					base_x.wrapping_add(1), base_y.wrapping_add(1), base_z,
					rel_x - 1. + 2. * *G3, rel_y - 1. + 2. * *G3, rel_z + 2. * *G3) }
			else if rel_x >= rel_y && rel_z >= rel_y {
				simplex_factor_3(&self.perm_table,
					base_x.wrapping_add(1), base_y, base_z.wrapping_add(1),
					rel_x - 1. + 2. * *G3, rel_y + 2. * *G3, rel_z - 1. + 2. * *G3) }
			else {
				simplex_factor_3(&self.perm_table,
					base_x, base_y.wrapping_add(1), base_z.wrapping_add(1),
					rel_x + 2. * *G3, rel_y - 1. + 2. * *G3, rel_z - 1. + 2. * *G3) }
			+
			simplex_factor_3(&self.perm_table,
				base_x.wrapping_add(1), base_y.wrapping_add(1), base_z.wrapping_add(1),
				rel_x - 1. + 3. * *G3, rel_y - 1. + 3. * *G3, rel_z - 1. + 3. * *G3))
	}
}

fn simplex_factor_4(perm_table: &PermTable, base_w: u8, base_x: u8, base_y: u8, base_z: u8, rel_w: f64, rel_x: f64, rel_y: f64, rel_z: f64) -> f64 {
	let t = 0.6 - rel_w.powi(2) - rel_x.powi(2) - rel_y.powi(2) - rel_z.powi(2);
	if t < 0. { 0.0 }
	else {
		let p = perm_table.0[base_w.wrapping_add(
			perm_table.0[base_x.wrapping_add(
				perm_table.0[base_y.wrapping_add(
					perm_table.0[base_z as usize]
				) as usize]
			) as usize]
		) as usize] % 12;
		let g = GRAD4[p as usize];
		t.powi(4) * (g[0] as f64 * rel_w + g[1] as f64 * rel_x + g[2] as f64 * rel_y + g[3] as f64 * rel_z)
	}
}

impl NoiseDomain<Vector<f64, 4>, f64> for PermSeeded<Simplex> {
	fn noise(&self, point: Vector<f64, 4>) -> f64 {
		let win = point[0];
		let xin = point[1];
		let yin = point[2];
		let zin = point[3];
		
		let s = (win + xin + yin + zin) * *F4;
		let base_w = (win + s).floor();
		let base_x = (xin + s).floor();
		let base_y = (yin + s).floor();
		let base_z = (zin + s).floor();
		
		let t = (base_w + base_x + base_y + base_z) * *G4;
		let rel_w = win - base_w + t;
		let rel_x = xin - base_x + t;
		let rel_y = yin - base_y + t;
		let rel_z = zin - base_z + t;
		
		let base_w = (base_w as i64).rem_euclid(256) as u8;
		let base_x = (base_x as i64).rem_euclid(256) as u8;
		let base_y = (base_y as i64).rem_euclid(256) as u8;
		let base_z = (base_z as i64).rem_euclid(256) as u8;
		
		return 27.0 * (
			simplex_factor_4(&self.perm_table,
				base_w, base_x, base_y, base_z,
				rel_w, rel_x, rel_y, rel_z)
			+
			if rel_w >= rel_x && rel_w >= rel_y && rel_w >= rel_z {
				simplex_factor_4(&self.perm_table,
					base_w.wrapping_add(1), base_x, base_y, base_z,
					rel_w - 1. + *G4, rel_x + *G4, rel_y + *G4, rel_z + *G4) }
			else if rel_x >= rel_w && rel_x >= rel_y && rel_x >= rel_z {
				simplex_factor_4(&self.perm_table,
					base_w, base_x.wrapping_add(1), base_y, base_z,
					rel_w + *G4, rel_x - 1. + *G4, rel_y + *G4, rel_z + *G4) }
			else if rel_y >= rel_w && rel_y >= rel_x && rel_y >= rel_z {
				simplex_factor_4(&self.perm_table,
					base_w, base_x, base_y.wrapping_add(1), base_z,
					rel_w + *G4, rel_x + *G4, rel_y - 1. + *G4, rel_z + *G4) }
			else {
				simplex_factor_4(&self.perm_table,
					base_w, base_x, base_y, base_z.wrapping_add(1),
					rel_w + *G4, rel_x + *G4, rel_y + *G4, rel_z - 1. + *G4) }
			+
			if rel_w >= rel_y && rel_w >= rel_z && rel_x >= rel_y && rel_x >= rel_z {
				simplex_factor_4(&self.perm_table,
					base_w.wrapping_add(1), base_x.wrapping_add(1), base_y, base_z,
					rel_w - 1. + 2. * *G4, rel_x - 1. + 2. * *G4, rel_y + 2. * *G4, rel_z + 2. * *G4) }
			else if rel_w >= rel_x && rel_w >= rel_z && rel_y >= rel_x && rel_y >= rel_z {
				simplex_factor_4(&self.perm_table,
					base_w.wrapping_add(1), base_x, base_y.wrapping_add(1), base_z,
					rel_w - 1. + 2. * *G4, rel_x + 2. * *G4, rel_y - 1. + 2. * *G4, rel_z + 2. * *G4) }
			else if rel_w >= rel_y && rel_w >= rel_x && rel_z >= rel_y && rel_z >= rel_x {
				simplex_factor_4(&self.perm_table,
					base_w.wrapping_add(1), base_x, base_y, base_z.wrapping_add(1),
					rel_w - 1. + 2. * *G4, rel_x + 2. * *G4, rel_y + 2. * *G4, rel_z - 1. + 2. * *G4) }
			else if rel_y >= rel_w && rel_y >= rel_z && rel_x >= rel_w && rel_x >= rel_z {
				simplex_factor_4(&self.perm_table,
					base_w, base_x.wrapping_add(1), base_y.wrapping_add(1), base_z,
					rel_w + 2. * *G4, rel_x - 1. + 2. * *G4, rel_y - 1. + 2. * *G4, rel_z + 2. * *G4) }
			else if rel_z >= rel_y && rel_z >= rel_w && rel_x >= rel_y && rel_x >= rel_w {
				simplex_factor_4(&self.perm_table,
					base_w, base_x.wrapping_add(1), base_y, base_z.wrapping_add(1),
					rel_w + 2. * *G4, rel_x - 1. + 2. * *G4, rel_y + 2. * *G4, rel_z - 1. + 2. * *G4) }
			else {
				simplex_factor_4(&self.perm_table,
					base_w, base_x, base_y.wrapping_add(1), base_z.wrapping_add(1),
					rel_w + 2. * *G4, rel_x + 2. * *G4, rel_y - 1. + 2. * *G4, rel_z - 1. + 2. * *G4) }
			+
			if rel_w >= rel_z && rel_x >= rel_z && rel_y >= rel_z {
				simplex_factor_4(&self.perm_table,
					base_w.wrapping_add(1), base_x.wrapping_add(1), base_y.wrapping_add(1), base_z,
					rel_w - 1. + 3. * *G4, rel_x - 1. + 3. * *G4, rel_y - 1. + 3. * *G4, rel_z + 3. * *G4) }
			else if rel_w >= rel_y && rel_x >= rel_y && rel_z >= rel_y {
				simplex_factor_4(&self.perm_table,
					base_w.wrapping_add(1), base_x.wrapping_add(1), base_y, base_z.wrapping_add(1),
					rel_w - 1. + 3. * *G4, rel_x - 1. + 3. * *G4, rel_y + 3. * *G4, rel_z - 1. + 3. * *G4) }
			else if rel_w >= rel_x && rel_y >= rel_x && rel_z >= rel_x {
				simplex_factor_4(&self.perm_table,
					base_w.wrapping_add(1), base_x, base_y.wrapping_add(1), base_z.wrapping_add(1),
					rel_w - 1. + 3. * *G4, rel_x + 3. * *G4, rel_y - 1. + 3. * *G4, rel_z - 1. + 3. * *G4) }
			else {
				simplex_factor_4(&self.perm_table,
					base_w, base_x.wrapping_add(1), base_y.wrapping_add(1), base_z.wrapping_add(1),
					rel_w + 3. * *G4, rel_x - 1. + 3. * *G4, rel_y - 1. + 3. * *G4, rel_z - 1. + 3. * *G4) }
			+
			simplex_factor_4(&self.perm_table,
				base_w.wrapping_add(1), base_x.wrapping_add(1), base_y.wrapping_add(1), base_z.wrapping_add(1),
				rel_w - 1. + 4. * *G4, rel_x - 1. + 4. * *G4, rel_y - 1. + 4. * *G4, rel_z - 1. + 4. * *G4))
	}
}
