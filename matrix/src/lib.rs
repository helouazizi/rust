use std::ops::{Add, Div, Mul, Sub};
pub trait Scalar : Copy + Add + Sub + Mul + Div {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}


impl Scalar for i32 {
    type Item = i32;

    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}

impl Scalar for f64 {
    type Item = f64;

    fn zero() -> Self::Item { 0.0 }
    fn one() -> Self::Item { 1.0 }
}

impl Scalar for u32 {
    type Item = u32;

    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}


impl Scalar for u64 {
    type Item = u64;

    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}

impl Scalar for i64 {
    type Item = i64;

    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}


impl Scalar for f32 {
    type Item = f32;

    fn zero() -> Self::Item { 0.0 }
    fn one() -> Self::Item { 1.0 }
}

#[derive(Debug,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero();col];row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut  res =  Matrix(vec![vec![T::zero();n];n]);
        for i in 0..n  {
            res[i][i] = T::one();
        }
        res
	}
}


use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        &self.0[row] 
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.0[row]
    }
}
