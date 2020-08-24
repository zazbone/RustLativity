use crate::_utils::Sqrt;


use std::ops::{
    Div,
    Mul,
    SubAssign
};


use ndarray;


#[derive(Debug, PartialEq)]
pub struct QuadVector<T> {
    pub _w: T,
    pub r: ndarray::Array1<T>
}


pub trait Accessor<T> {
    fn w(&self) -> T;

    fn x(&self) -> T;

    fn y(&self) -> T;

    fn z(&self) -> T;
}

pub trait Init<T> {
    fn new(w: T, x: T, y: T, z: T) -> Self;
}

pub trait LinearAlgebra<T> {
    fn dot(&self, other: &Self) -> T;

    fn norm(&self) -> T;

    fn normelized(&self) -> T;
}


impl<T> Accessor<T> for QuadVector<T> where
    T: Copy
{
    fn w(&self) -> T {
        self._w
    }

    fn x(&self) -> T {
        self.r[0]
    }

    fn y(&self) -> T {
        self.r[1]
    }

    fn z(&self) -> T {
        self.r[2]
    }
}

impl<T> Init<T> for QuadVector<T> where
    T: Clone
{
    fn new(w: T, x: T, y: T, z: T) -> Self {
        Self {
            _w: w,
            r: ndarray::array![x, y, z]
         }
    }
}

impl<T> LinearAlgebra<T> for QuadVector<T> where
    T: Copy + Mul<Output = T> + SubAssign + Sqrt<T>
{
    fn dot(&self, other: &Self) -> T {
        let mut out = self._w * other._w;
        for i in 0..3 {
            let tmp = self.r[i] * other.r[i];
            out -= tmp;
        }
        out
    }

    fn norm(&self) -> T {
        match Sqrt::n_sqrt(&self.dot(self)) {
            Ok(value) => value,
            Err(_) => panic!("Invalide value")
        }
    }

    fn normelized(&self) -> T {
        self.w()
    }
}


impl<T> Div::<T> for &QuadVector<T> where
    T: Div<T, Output=T>
{
    type Output = QuadVector<T>;

    fn div(self, other: T) -> Self::Output {
        Self::Output {
            _w: self._w / other,
            r: self.r
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    use assert_approx_eq::assert_approx_eq;


    #[test]
    fn init() {
        assert_eq!(
            QuadVector{
                _w: 10,
                r: ndarray::arr1(&[3, 5, 1203])
            },
            QuadVector::new(10, 3, 5, 1203)
        );
        assert_ne!(
            QuadVector{
                _w: 123,
                r: ndarray::arr1(&[36, 52, 12303])
            },
            QuadVector::new(12, 3, 5, 1203)
        )

    }

    #[test]
    fn dot_product() {
        let quad = QuadVector::new(2.4f64, 2.8f64, 12.00f64, 43.12f64);
        assert_approx_eq!(
            quad.dot(&quad),
            -2005.4144f64,
            1e-3f64
        );
        assert_approx_eq!(
            quad.dot(&QuadVector::new(1.0f64, 1.0f64, 1.0f64, 1.0f64)),
            -55.52f64,
            1e-3f64
        )
    }
}
