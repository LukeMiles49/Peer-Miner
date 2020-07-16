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

impl<TInner: Copy, const N: usize> Octaves<TInner, N> where {
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

/*
impl<TInner: Copy, const N: usize> Octaves<TInner, N> {
	pub fn new(value: TInner, lacunarity: f64, persistence: f64) -> Self {
		let mut octaves: [MaybeUninit<ScaleNoise<TInner, f64, f64>>; N] = unsafe { MaybeUninit::uninit().assume_init() };
		
		for i in 0..N {
			octaves[i] = MaybeUninit::new(ScaleNoise::new(value, lacunarity.powi(i as i32), persistence.powi(i as i32)));
		}
		
		Self {
			contents: SumNoise::new(unsafe { std::mem::transmute_copy(&octaves) })
		}
	}
}

impl<TSeed: Split, TConfig: NoiseConfig<TSeed::TSplit>, const N: usize> NoiseConfig<TSeed> for Octaves<TConfig, N> where SumNoise<ScaleNoise<TConfig, f64, f64>, N>: NoiseConfig<TSeed, TNoise = SumNoise<ScaleNoise<TConfig::TNoise, f64, f64>, N>>, Octaves<TConfig::TNoise, N>: Noise {
	type TNoise = Octaves<TConfig::TNoise, N>;
	
	fn create(self, seed: TSeed) -> Self::TNoise {
		Self::TNoise {
			contents: self.contents.create(seed),
		}
	}
}

impl<TNoise, const N: usize> Noise for Octaves<TNoise, N> where SumNoise<ScaleNoise<TNoise, f64, f64>, N>: Noise {
	type TValue = <SumNoise<ScaleNoise<TNoise, f64, f64>, N> as Noise>::TValue;
}

impl<TNoise, TArg, const N: usize> NoiseDomain<TArg> for Octaves<TNoise, N> where SumNoise<ScaleNoise<TNoise, f64, f64>, N>: NoiseDomain<TArg> {
	fn noise(&self, arg: TArg) -> Self::TValue {
		self.contents.noise(arg)
	}
}
*/
