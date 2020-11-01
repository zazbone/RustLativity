use crate::{
    math::{
        traits::*,
        matrix::*
    },
    alg::base::four_vector::*
};

use std::ops::{
    // Accessors
    Index, IndexMut
};


#[derive(Copy, Clone)]
pub struct FourPosition<T>([T; 4]);


impl<T> FourPosition<T>
where T: Copy
{
    pub fn ct(&self) -> T {
        self[0]
    }
    pub fn x(&self) -> T {
        self[1]
    }
    pub fn y(&self) -> T {
        self[2]
    }
    pub fn z(&self) -> T {
        self[3]
    }
}


impl<T> Init<T> for FourPosition<T>
{
    fn new(w: T, x: T, y: T, z: T) -> Self {
        Self{0: [w, x, y, z]}
    }
}

impl<T> LinearAlgebra<T> for FourPosition<T>
where T: Copy + Sqrt + MathOps + Values
{
    const METRIC: LorentzMatrix<T> = LorentzMatrix{0:[
        [T::ONE,  T::ZERO,    T::ZERO,    T::ZERO],
        [T::ZERO, T::NEQ_ONE, T::ZERO,    T::ZERO],
        [T::ZERO, T::ZERO,    T::NEQ_ONE, T::ZERO],
        [T::ZERO, T::ZERO,    T::ZERO,    T::NEQ_ONE]
        ]};
}


impl<T> Index<usize> for FourPosition<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for FourPosition<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
