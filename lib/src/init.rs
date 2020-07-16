use std::mem::MaybeUninit;

pub trait Init<T, I> {
	fn init<F: Fn(I) -> T>(elem: F) -> Self;
}

impl<T, const N: usize> Init<T, usize> for [T; N] {
	fn init<F: Fn(usize) -> T>(elem: F) -> Self {
		let mut contents: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
		
		for i in 0..N {
			contents[i] = MaybeUninit::new(elem(i));
		}
		
		// FIXME: Replace with transmute once it works with const generic array sizes
		unsafe { std::mem::transmute_copy(&contents) }
	}
}

impl<T, const N1: usize, const N2: usize> Init<T, [usize; 2]> for [[T; N1]; N2] {
	fn init<F: Fn([usize; 2]) -> T>(elem: F) -> Self {
		Self::init(|i2| <[T; N1]>::init(|i1| elem([i1, i2])))
	}
}

impl<T, const N1: usize, const N2: usize, const N3: usize> Init<T, [usize; 3]> for [[[T; N1]; N2]; N3] {
	fn init<F: Fn([usize; 3]) -> T>(elem: F) -> Self {
		Self::init(|i3| <[[T; N1]; N2]>::init(|[i1, i2]: [usize; 2]| elem([i1, i2, i3])))
	}
}

impl<T, const N1: usize, const N2: usize, const N3: usize, const N4: usize> Init<T, [usize; 4]> for [[[[T; N1]; N2]; N3]; N4] {
	fn init<F: Fn([usize; 4]) -> T>(elem: F) -> Self {
		Self::init(|i4| <[[[T; N1]; N2]; N3]>::init(|[i1, i2, i3]: [usize; 3]| elem([i1, i2, i3, i4])))
	}
}

impl<T, const N1: usize, const N2: usize, const N3: usize, const N4: usize, const N5: usize> Init<T, [usize; 5]> for [[[[[T; N1]; N2]; N3]; N4]; N5] {
	fn init<F: Fn([usize; 5]) -> T>(elem: F) -> Self {
		Self::init(|i5| <[[[[T; N1]; N2]; N3]; N4]>::init(|[i1, i2, i3, i4]: [usize; 4]| elem([i1, i2, i3, i4, i5])))
	}
}

impl<T, const N1: usize, const N2: usize, const N3: usize, const N4: usize, const N5: usize, const N6: usize> Init<T, [usize; 6]> for [[[[[[T; N1]; N2]; N3]; N4]; N5]; N6] {
	fn init<F: Fn([usize; 6]) -> T>(elem: F) -> Self {
		Self::init(|i6| <[[[[[T; N1]; N2]; N3]; N4]; N5]>::init(|[i1, i2, i3, i4, i5]: [usize; 5]| elem([i1, i2, i3, i4, i5, i6])))
	}
}
