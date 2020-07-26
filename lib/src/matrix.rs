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
	Inv, Pow,
	MulAdd, MulAddAssign,
};

use num::integer::Integer;

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
	
	pub fn swap_row(&mut self, i: usize, j: usize) {
		for k in 0..N {
			self.contents[k].swap(i, j);
		}
	}
	
	pub fn swap_col(&mut self, i: usize, j: usize) {
		self.contents.swap(i, j);
	}
}

pub trait Scalar { }
impl<T, const M: usize, const N: usize> !Scalar for Matrix<T, M, N> { }
impl<T: Num> Scalar for T { }

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


impl<T: Copy, const M: usize, const N: usize> Zero for Matrix<T, M, N> where
	T: Zero,
{
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

impl<T: Copy, const N: usize> One for Matrix<T, N, N> where
	T: Zero + One + MulAdd<Output = T>,
{
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

impl<T, const M: usize, const N: usize> PartialEq for Matrix<T, M, N> where
	T: PartialEq,
{
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

// TODO: Define operations that can be performed in place in terms of the _Assign trait

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const N: usize> Mul<TRhs> for Matrix<TLhs, M, N> where
	TLhs: Mul<TRhs, Output = TOutput>,
	TRhs: Scalar,
{
	type Output = Matrix<TOutput, M, N>;
	
	fn mul(self, rhs: TRhs) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[row, col]] * rhs
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> MulAssign<TRhs> for Matrix<TLhs, M, N> where
	Self: Mul<TRhs, Output = Matrix<TLhs, M, N>>,
	TRhs: Scalar,
{
	fn mul_assign(&mut self, rhs: TRhs) {
		take(self, |s| s.mul(rhs));
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const N: usize> Div<TRhs> for Matrix<TLhs, M, N> where
	TLhs: Div<TRhs, Output = TOutput>,
	TRhs: Scalar,
{
	type Output = Matrix<TOutput, M, N>;
	
	fn div(self, rhs: TRhs) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[row, col]] / rhs
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> DivAssign<TRhs> for Matrix<TLhs, M, N> where
	Self: Div<TRhs, Output = Matrix<TLhs, M, N>>,
	TRhs: Scalar,
{
	fn div_assign(&mut self, rhs: TRhs) {
		take(self, |s| s.div(rhs));
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const N: usize> Add<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where
	TLhs: Add<TRhs, Output = TOutput>,
{
	type Output = Matrix<TOutput, M, N>;
	
	fn add(self, rhs: Matrix<TRhs, M, N>) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[row, col]] + rhs[[row, col]]
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> AddAssign<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where
	Self: Add<Matrix<TRhs, M, N>, Output = Self>,
{
	fn add_assign(&mut self, rhs: Matrix<TRhs, M, N>) {
		take(self, |s| s.add(rhs));
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const N: usize> Sub<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where
	TLhs: Sub<TRhs, Output = TOutput>,
{
	type Output = Matrix<TOutput, M, N>;
	
	fn sub(self, rhs: Matrix<TRhs, M, N>) -> Self::Output {
		Self::Output::init(|[row, col]| {
			self[[row, col]] - rhs[[row, col]]
		})
	}
}

impl<T: Copy, TOutput, const M: usize, const N: usize> Neg for Matrix<T, M, N> where
	T: Neg<Output = TOutput>,
{
	type Output = Matrix<TOutput, M, N>;
	
	fn neg(self) -> Self::Output {
		Self::Output::init(|[row, col]| {
			-self[[row, col]]
		})
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> SubAssign<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where
	Self: Sub<Matrix<TRhs, M, N>, Output = Self>,
{
	fn sub_assign(&mut self, rhs: Matrix<TRhs, M, N>) {
		take(self, |s| s.sub(rhs));
	}
}

impl<TLhs: Copy, TRhs: Copy, TOutput, const M: usize, const K: usize, const N: usize> Mul<Matrix<TRhs, K, N>> for Matrix<TLhs, M, K> where
	TLhs: Mul<TRhs, Output = TOutput> + MulAdd<TRhs, TOutput, Output = TOutput>,
	TOutput: Zero,
{
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

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> MulAssign<Matrix<TRhs, M, N>> for Matrix<TLhs, M, N> where
	Self: Mul<Matrix<TRhs, M, N>, Output = Self>,
{
	fn mul_assign(&mut self, rhs: Matrix<TRhs, M, N>) {
		take(self, |s| s.mul(rhs));
	}
}

impl<TLhs: Copy, TA: Copy, TB: Copy, const M: usize, const K: usize, const N: usize> MulAdd<Matrix<TA, K, N>, Matrix<TB, M, N>> for Matrix<TLhs, M, K> where
	TLhs: MulAdd<TA, TB, Output = TB>,
{
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

impl<TLhs: Copy, TA: Copy, TB: Copy, const M: usize, const N: usize> MulAddAssign<Matrix<TA, M, N>, Matrix<TB, M, N>> for Matrix<TLhs, M, N> where
	Self: MulAdd<Matrix<TA, M, N>, Matrix<TB, M, N>, Output = Self>,
{
	fn mul_add_assign(&mut self, a: Matrix<TA, M, N>, b: Matrix<TB, M, N>) {
		take(self, |s| s.mul_add(a, b));
	}
}

impl<TLhs: Copy, TRhs: Copy, const M: usize, const N: usize> Div<Matrix<TRhs, N, N>> for Matrix<TLhs, M, N> where
	TLhs: MulAdd<TRhs, TLhs, Output = TLhs> + DivAssign<TRhs>,
	TRhs: Zero /*+ One*/ + MulAdd<TRhs, TRhs, Output = TRhs> + DivAssign<TRhs> + Neg<Output = TRhs>,
{
	type Output = Self;
	
	fn div(mut self, mut rhs: Matrix<TRhs, N, N>) -> Self::Output {
		for i in 0..N {
			if let Some(j) = (i..N).find(|j| !rhs[[*j, *j]].is_zero()) {
				if i != j {
					rhs.swap_row(i, j);
					self.swap_row(i, j);
				}
				
				let factor = rhs[[i, i]];
				// Never going to use this element again, so can skip this
				// rhs[[i, i]] = T::one();
				for k in i+1..N {
					rhs[[i, k]] /= factor;
				}
				for k in 0..N {
					self[[i, k]] /= factor;
				}
				
				for j in (0..i).chain(i+1..N) {
					let factor = -rhs[[j, i]];
					// Never going to use this element again, so can skip this
					// rhs[[j, i]] = T::zero();
					for k in i+1..N {
						rhs[[j, k]] = rhs[[i, k]].mul_add(factor, rhs[[j, k]]);
					}
					for k in 0..N {
						self[[j, k]] = self[[i, k]].mul_add(factor, self[[j, k]]);
					}
				}
			} else {
				panic!("Matrix has no inverse")
			}
		}
		
		self
	}
}

impl<T: Copy, const M: usize, const N: usize> DivAssign<Matrix<T, N, N>> for Matrix<T, M, N> where
	Self: Div<Matrix<T, N, N>, Output = Self>,
{
	fn div_assign(&mut self, rhs: Matrix<T, N, N>) {
		take(self, |s| s.div(rhs));
	}
}

impl<T: Copy, const N: usize> Inv for Matrix<T, N, N> where
	Self: One + Div<Self, Output = Self>,
{
	type Output = Self;
	
	fn inv(self) -> Self::Output {
		Self::one().div(self)
	}
}

// FIXME: Remove the Inv bound for Unsigned once Signed and Unsigned are mutually exclusive
impl<T: Copy, TRhs, const N: usize> Pow<TRhs> for Matrix<T, N, N> where
	Self: Inv<Output = Self> + One + MulAssign<Self>,
	TRhs: Integer,
{
	type Output = Self;
	
	fn pow(mut self, mut rhs: TRhs) -> Self::Output {
		if rhs < TRhs::zero() {
			self = self.inv();
			rhs = TRhs::zero() - rhs;
		}
		
		let mut result = Self::one();
		
		while rhs > TRhs::zero() {
			let (div, rem) = rhs.div_mod_floor(&(TRhs::one() + TRhs::one()));
			rhs = div;
			if rem == TRhs::one() {
				result *= self;
			}
		}
		
		result
	}
}
