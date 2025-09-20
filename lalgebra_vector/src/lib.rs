use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar : Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Vector<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self(data)
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut res = T::zero();
        for i in 0..self.0.len() {
            res = res + self.0[i] * other.0[i];
        }
        Some(res)
    }
}

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            panic!("Vector size mismatch");
        }
        let data = self.0.into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Vector(data)
    }
}