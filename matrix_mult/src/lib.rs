use std::ops::{Add, Div, Mul, Sub};
pub trait Scalar: Copy + Add + Sub + Mul + Div {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl Scalar for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut res = Matrix(vec![vec![T::zero(); n]; n]);
        for i in 0..n {
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

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0.get(n).unwrap().to_vec()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res: Vec<T> = Vec::new();
        for row in self.0.clone() {
            res.push(row[n].clone());
        }
        res
    }
}

impl<T> Mul for Matrix<T>
where
    T: Scalar + Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Option<self::Matrix<T>>;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();
        let mut result = Matrix::zero(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = T::zero();
                for k in 0..self.number_of_cols() {
                    sum = sum + self[i][k] * rhs[k][j];
                }
                result[i][j] = sum;
            }
        }

        Some(result)
    }
}
