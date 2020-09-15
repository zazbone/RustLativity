use crate::algebra::quad::FourVector;


use std::ops::{
    Index,
    Add,
    Mul
};


/// Matrix 4x4 used for referential transform
pub struct LorentzMatrix<T> {
    pub content: [[T; 4]; 4]
}


trait LinearAlgebra<T> {
    fn dot(&self, other: &FourVector<T>) -> FourVector<T>;
}


impl<T> LinearAlgebra<T> for LorentzMatrix<T> where
    T: Add<Output=T> + Mul<Output=T> + Copy
{
    fn dot(&self, other: &FourVector<T>) -> FourVector<T> {
        let coordinate: [T; 4] = [
            matrix4_scalar_product(&self[0], other[0]),
            matrix4_scalar_product(&self[1], other[1]),
            matrix4_scalar_product(&self[2], other[2]),
            matrix4_scalar_product(&self[3], other[3])
        ];
        FourVector {
            representation: other.representation,
            coordinate: coordinate,
            signature: other.signature
        }
    }
}

fn matrix4_scalar_product<T>(array4: &[T; 4], scalar: T) -> T where
    T: Add<Output=T> + Mul<Output=T> + Copy
{
    array4[0] * scalar + array4[1] * scalar + array4[2] * scalar + array4[3] * scalar
}


impl<T> Index<usize> for &LorentzMatrix<T> {
    type Output = [T; 4];

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            // TODO: edit code when `https://github.com/rust-lang/rust/issues/37854` issues corrected
            0 | 1 | 2 | 3 => &self.content[i],
            _ => panic!("Index {} out of range", i)
        }
    }
}
