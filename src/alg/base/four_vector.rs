use std::ops::{
    // Accessors
    Index, IndexMut
};

use crate::math::{
    traits::{MathOps, Sqrt, Values},
    matrix::LorentzMatrix
};


/// Traits list that Fourvector Should implement \
/// Copy, Sized \
/// From std: Index IndexMut\
/// From the lib: Init, LinearAlgebra


pub trait Init<T>: Sized {
    fn new(w: T, x: T, y: T, z: T) -> Self;
}


pub trait LinearAlgebra<T>
where
    Self: Copy + Sized + Index<usize, Output=T> + IndexMut<usize>,
    T: MathOps + Sqrt + Copy + Values
{
    const METRIC: LorentzMatrix<T>;

    fn transformed(mut self, mat: LorentzMatrix<T>) -> Self {
        for i in 0..4 {
            let mut  result = T::ZERO;
            for j in 0..4 {
                result += self[i] * mat[(i, j)];
            }
            self[i] = result;
        }
        self
    }

    fn transform(&mut self, mat: LorentzMatrix<T>) {
        // not enought iterator
        for i in 0..4 {
            let mut result = T::ZERO;
            for j in 0..4 {
                result += self[i] * mat[(i, j)];
            }
            self[i] = result;
        }
    }

    fn dot(mut self, other: &Self) -> T {
        self.transform(Self::METRIC);
        self[0] * other[0] -
        self[1] * other[1] -
        self[2] * other[2] -
        self[3] * other[3]
    }

    fn norm(&self) -> T {
        let out:T = self.dot(self);
        out.norm_sqrt().expect("Unexpected error")
    }
}
