use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait NoiseConfig: Copy {
	type TSeed;
	type TNoise: Noise;
	
	fn seed(self, seed: Self::TSeed) -> Self::TNoise;
}

pub trait Noise { }

pub trait NoiseDomain<TArg, TValue>: Noise {
	fn noise(&self, arg: TArg) -> TValue;
}

pub trait Split {
	fn split(&self, n: usize) -> Self;
}

impl Split for u64 {
	fn split(&self, n: usize) -> Self {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		n.hash(&mut hasher);
		hasher.finish()
	}
}

pub type Seeded<TConfig> = <TConfig as NoiseConfig>::TNoise;
