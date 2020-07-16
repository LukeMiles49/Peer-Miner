use super::{
	Noise,
	NoiseConfig,
	NoiseDomain,
};

use std::{
	marker::PhantomData,
	ops::Mul,
};

pub struct ScaleNoise<TInner, TInScale: Copy, TInnerValue, TOutScale: Copy> {
	contents: TInner,
	scale_in: TInScale,
	scale_out: TOutScale,
	
	__phantom: PhantomData<fn() -> TInnerValue>
}

// Need to implement these manually because it doesn't seem to work properly if TInnerValue isn't Copy

impl<TInner: Copy, TInScale: Copy, TInnerValue, TOutScale: Copy> Copy for ScaleNoise<TInner, TInScale, TInnerValue, TOutScale> { }

impl<TInner: Clone, TInScale: Copy, TInnerValue, TOutScale: Copy> Clone for ScaleNoise<TInner, TInScale, TInnerValue, TOutScale> {
	fn clone(&self) -> Self {
		Self {
			contents: self.contents.clone(),
			scale_in: self.scale_in,
			scale_out: self.scale_out,
			
			__phantom: PhantomData,
		}
	}
}

impl<TInner, TInScale: Copy, TInnerValue, TOutScale: Copy> ScaleNoise<TInner, TInScale, TInnerValue, TOutScale> {
	pub fn new(contents: TInner, scale_in: TInScale, scale_out: TOutScale) -> Self {
		Self {
			contents,
			scale_in,
			scale_out,
			
			__phantom: PhantomData,
		}
	}
}

impl<TInner: Copy, TInScale: Copy, TInnerValue, TOutScale: Copy> NoiseConfig for ScaleNoise<TInner, TInScale, TInnerValue, TOutScale> where
	TInner: NoiseConfig,
{
	type TSeed = TInner::TSeed;
	type TNoise = ScaleNoise<TInner::TNoise, TInScale, TInnerValue, TOutScale>;
	
	fn seed(self, seed: Self::TSeed) -> Self::TNoise {
		Self::TNoise {
			contents: self.contents.seed(seed),
			scale_in: self.scale_in,
			scale_out: self.scale_out,
			
			__phantom: PhantomData,
		}
	}
}

impl<TInner, TInScale: Copy, TInnerValue, TOutScale: Copy> Noise for ScaleNoise<TInner, TInScale, TInnerValue, TOutScale> where
	TInner: Noise
{ }

impl<TInner, TArg, TInScale: Copy, TInnerArg, TInnerValue, TOutScale: Copy, TValue> NoiseDomain<TArg, TValue> for ScaleNoise<TInner, TInScale, TInnerValue, TOutScale> where
	TInner: NoiseDomain<TInnerArg, TInnerValue>,
	TArg: Mul<TInScale, Output = TInnerArg>,
	TInnerValue: Mul<TOutScale, Output = TValue>,
{
	fn noise(&self, arg: TArg) -> TValue {
		self.contents.noise(arg * self.scale_in) * self.scale_out
	}
}
