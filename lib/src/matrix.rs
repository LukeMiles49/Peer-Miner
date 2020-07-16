use super::Init;

use std::ops::{
	Add, AddAssign,
	Sub, SubAssign,
	Neg,
	Mul, MulAssign,
	Div, DivAssign,
	Index, IndexMut,
};

use num::traits::{
	Num,
	One, Zero,
	// TODO Inv, Pow,
	MulAdd, MulAddAssign,
};

use take_mut::take;


#[derive(Copy, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
	contents: [[T; M]; N],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn new(contents: [[T; M]; N]) -> Self {
		Self {
			contents,
		}
	}
}

// FIXME
pub trait NotMatrix<T, const N: usize> { }
impl<T, const M: usize, const N: usize> !NotMatrix<T, N> for Matrix<T, M, N> { }
impl<TNum: Num, T, const N: usize> NotMatrix<T, N> for TNum { }

impl<T, const M: usize, const N: usize> Init<T, [usize; 2]> for Matrix<T, M, N> {
	fn init<F: Fn([usize; 2]) -> T>(elem: F) -> Self {
		Self::new(<[[T; M]; N]>::init(elem))
	}
}

impl<T, const M: usize, const N: usize> Index<[usize; 2]> for Matrix<T, M, N> {
	type Output = T;
	
	fn index(&self, [row, col]: [usize; 2]) -> &T {
		&self.contents[col][row]
	}
}

impl<T, const M: usize, const N: usize> IndexMut<[usize; 2]> for Matrix<T, M, N> {
	fn index_mut(&mut self, [row, col]: [usize; 2]) -> &mut T {
		&mut self.contents[col][row]
	}
}

#[macro_export]
macro_rules! matrix {
	($( $( $x: expr ),*);*) => {
		{
			use $crate::Matrix;
			Matrix::new([ $( [ $($x),* ] ),* ]).transpose()
		}
	}
}


pub type Vector<T, const M: usize> = Matrix<T, M, 1>;

impl<T, const M: usize> Vector<T, M> {
	pub fn v_new(contents: [T; M]) -> Self {
		Self::new([contents])
	}
	
	pub fn v_init<F: Fn(usize) -> T>(elem: F) -> Self {
		Self::init(|[row, _]| elem(row))
	}
}

impl<T, const M: usize> Index<usize> for Vector<T, M> {
	type Output = T;
	
	fn index(&self, index: usize) -> &T {
		&self[[index, 0]]
	}
}

impl<T, const M: usize> IndexMut<usize> for Vector<T, M> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		&mut self[[0, index]]
	}
}

impl<T, const M: usize> From<[T; M]> for Vector<T, M> {
	fn from(item: [T; M]) -> Self {
		Self::v_new(item)
	}
}

#[macro_export]
macro_rules! vector {
	($( $x: expr ),*) => {
		{
			use $crate::Vector;
			Vector::v_new([ $( $x ),* ])
		}
	}
}


impl<T: Copy, const M: usize, const N: usize> Zero for Matrix<T, M, N> where T: Zero {
	fn zero() -> Self {
		Self::init(|_| T::zero())
	}
	
	fn is_zero(&self) -> bool {
		for col in 0..N {
			for row in 0..M {
				if !self[[row, col]].is_zero() {
					return false
				}
			}
		}
		true
	}
}

impl<T: Copy, const N: usize> One for Matrix<T, N, N> where T: Zero + One + MulAdd<Output = T> {
	fn one() -> Self {
		Self::init(|[row, col]| if row == col { T::one() } else { T::zero() })
	}
	
	// FIXME: Uncomment when One relaxes the PartialEq requirement
	
	/*
	fn is_one(&self) -> bool {
		for col in 0..N {
			for row in 0..N {
				if
					if col == row { self.contents[[row, col]].is_one() }
					else { self[[row, col]].is_zero() }
				{
					return false
				}
			}
		}
		true
	}
	*/
}

impl<T, const M: usize, const N: usize> PartialEq for Matrix<T, M, N> where T: PartialEq {
	fn eq(&self, rhs: &Matrix<T, M, N>) -> bool {
		for col in 0..N {
			for row in 0..M {
				if self[[row, col]] != rhs[[row, col]] {
					return false
				}
			}
		}
		true
	}
}

trait Transpose {
	type Output;
	
	fn transpose(self) -> Self::Output;
}

impl<T: Copy, const M: usize, const N: usize> Transpose for Matrix<T, M, N> {
	type Output = Matrix<T, N, M>;
	
	fn transpose(self) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[col, row]]
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const N: usize> Mul<TRhs> for Matrix<TLhs, M, N> where TLhs: Mul<TRhs, Output = TOutput>, TRhs: NotMatrix<TRhs, N> {
	type Output = Matrix<TOutput, M, N>;
	
	fn mul(self, rhs: TRhs) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[row, col]] * rhs
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> MulAssign<TRhs> for Matrix<TLhs, M, N> where Self: Mul<TRhs, Output = Matrix<TLhs, M, N>>, TRhs: NotMatrix<TRhs, N> {
	fn mul_assign(&mut self, rhs: TRhs) {
		take(self, |s| s.mul(rhs));
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const N: usize> Div<TRhs> for Matrix<TLhs, M, N> where TLhs: Div<TRhs, Output = TOutput> {
	type Output = Matrix<TOutput, M, N>;
	
	fn div(self, rhs: TRhs) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[row, col]] / rhs
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> DivAssign<TRhs> for Matrix<TLhs, M, N> where Self: Div<TRhs, Output = Matrix<TLhs, M, N>> {
	fn div_assign(&mut self, rhs: TRhs) {
		take(self, |s| s.div(rhs));
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const N: usize> Add<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where TLhs: Add<TRhs, Output = TOutput> {
	type Output = Matrix<TOutput, M, N>;
	
	fn add(self, rhs: Matrix<TRhs, M, N>) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[row, col]] + rhs[[row, col]]
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> AddAssign<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where Self: Add<Matrix<TRhs, M, N>, Output = Self> {
	fn add_assign(&mut self, rhs: Matrix<TRhs, M, N>) {
		take(self, |s| s.add(rhs));
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const N: usize> Sub<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where TLhs: Sub<TRhs, Output = TOutput> {
	type Output = Matrix<TOutput, M, N>;
	
	fn sub(self, rhs: Matrix<TRhs, M, N>) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[row, col]] - rhs[[row, col]]
		})
	}
}

impl<T: Copy, TOutput, const M: usize, const N: usize> Neg for Matrix<T, M, N> where T: Neg<Output = TOutput> {
	type Output = Matrix<TOutput, M, N>;
	
	fn neg(self) -> Self::Output {
		Self::Output::init(|[row, col]| {
			-self[[row, col]]
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> SubAssign<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where Self: Sub<Matrix<TRhs, M, N>, Output = Self> {
	fn sub_assign(&mut self, rhs: Matrix<TRhs, M, N>) {
		take(self, |s| s.sub(rhs));
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const K: usize, const N: usize> Mul<Matrix<TRhs, K, N>> for Matrix<TLhs, M, K> where TLhs: Mul<TRhs, Output = TOutput> + MulAdd<TRhs, TOutput, Output = TOutput>, TOutput: Zero {
	type Output = Matrix<TOutput, M, N>;
	
	fn mul(self, rhs: Matrix<TRhs, K, N>) -> Self::Output {
		Self::Output::init(|[row, col]| {
			let mut result = TOutput::zero();
			for k in 0..K {
				result = self.contents[k][row].mul_add(rhs.contents[col][k], result)
			}
			result
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> MulAssign<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where Self: Mul<Matrix<TRhs, M, N>, Output = Self> {
	fn mul_assign(&mut self, rhs: Matrix<TRhs, M, N>) {
		take(self, |s| s.mul(rhs));
	}
}

impl<TLhs: Copy, TA: Copy, TB: Copy, const M: usize, const K: usize, const N: usize> MulAdd<Matrix<TA, K, N>, Matrix<TB, M, N>> for Matrix<TLhs, M, K> where TLhs: MulAdd<TA, TB, Output = TB> {
	type Output = Matrix<TB, M, N>;
	
	fn mul_add(self, a: Matrix<TA, K, N>, b: Matrix<TB, M, N>) -> Self::Output {
		Self::Output::init(|[row, col]| {
			let mut result = b[[row, col]];
			for k in 0..K {
				result = self.contents[k][row].mul_add(a.contents[col][k], result)
			}
			result
		})
	}
}

impl<TLhs: Copy, TA: Copy, TB: Copy, const M: usize, const N: usize> MulAddAssign<Matrix<TA, M, N>, Matrix<TB, M, N>> for Matrix<TLhs, M, N> where Self: MulAdd<Matrix<TA, M, N>, Matrix<TB, M, N>, Output = Self> {
	fn mul_add_assign(&mut self, a: Matrix<TA, M, N>, b: Matrix<TB, M, N>) {
		take(self, |s| s.mul_add(a, b));
	}
}
