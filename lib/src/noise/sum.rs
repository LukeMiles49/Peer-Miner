use super::{
	super::Init,
	Noise,
	NoiseConfig,
	NoiseDomain,
	Split,
};

use std::ops::AddAssign;

use num::traits::Zero;

#[derive(Copy, Clone)]
pub struct SumNoise<TInner, const N: usize> {
	values: [TInner; N],
}

impl<TInner, const N: usize> SumNoise<TInner, N> {
	pub fn new(values: [TInner; N]) -> Self {
		Self {
			values,
		}
	}
}

// FIXME: Create without unsafe once const generics are stable and more widely supported

impl<TInner: Copy, const N: usize> NoiseConfig for SumNoise<TInner, N> where
	TInner: NoiseConfig,
	TInner::TSeed: Split,
{
	type TSeed = TInner::TSeed;
	type TNoise = SumNoise<TInner::TNoise, N>;
	
	fn seed(self, seed: Self::TSeed) -> Self::TNoise {
		Self::TNoise::new(<[TInner::TNoise; N]>::init(|i| self.values[i].seed(seed.split(i))))
	}
}

impl<TInner, const N: usize> Noise for SumNoise<TInner, N> where
	TInner: Noise
{ }

impl<TInner, TArg: Copy, TValue, const N: usize> NoiseDomain<TArg, TValue> for SumNoise<TInner, N> where
	TInner: NoiseDomain<TArg, TValue>,
	TValue: Zero + AddAssign,
{
	fn noise(&self, arg: TArg) -> TValue {
		let mut result = TValue::zero();
		
		for term in self.values.iter() {
			result += term.noise(arg);
		}
		
		result
	}
}
