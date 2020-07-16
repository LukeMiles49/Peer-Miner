use super::{
	super::Init,
	Noise,
	NoiseDomain,
	NoiseConfig,
	ScaleNoise,
	SumNoise,
};

#[derive(Copy, Clone)]
pub struct Octaves<TInner, const N: usize> {
	contents: SumNoise<ScaleNoise<TInner, f64, f64, f64>, N>,
}

impl<TInner: Copy, const N: usize> Octaves<TInner, N> {
	pub fn new(value: TInner, lacunarity: f64, persistence: f64) -> Self {
		Self {
			contents: SumNoise::new(<[ScaleNoise<TInner, f64, f64, f64>; N]>::init(|i| {
				ScaleNoise::new(value, lacunarity.powi(i as i32), persistence.powi(i as i32))
			}))
		}
	}
}

impl<TInner: Copy, TSeed, const N: usize> NoiseConfig for Octaves<TInner, N> where
	SumNoise<ScaleNoise<TInner, f64, f64, f64>, N>: NoiseConfig<TNoise = SumNoise<ScaleNoise<TInner::TNoise, f64, f64, f64>, N>, TSeed = TSeed>,
	TInner: NoiseConfig,
{
	type TSeed = TSeed;
	type TNoise = Octaves<TInner::TNoise, N>;
	
	fn seed(self, seed: Self::TSeed) -> Self::TNoise {
		Self::TNoise {
			contents: self.contents.seed(seed),
		}
	}
}

impl<TInner, const N: usize> Noise for Octaves<TInner, N> where
	SumNoise<ScaleNoise<TInner, f64, f64, f64>, N>: Noise
{ }

impl<TInner, TArg, TValue, const N: usize> NoiseDomain<TArg, TValue> for Octaves<TInner, N> where
	SumNoise<ScaleNoise<TInner, f64, f64, f64>, N>: NoiseDomain<TArg, TValue>
{
	fn noise(&self, arg: TArg) -> TValue {
		self.contents.noise(arg)
	}
}
