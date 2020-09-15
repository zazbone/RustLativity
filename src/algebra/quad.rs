use crate::utils::traits::Sqrt;

use std::ops::{
    // Operator
    Div,
    // Accessor
    Index,
    Mul,
    Neg,
    SubAssign,
};
use std::result::Result;
use std::string::String;

#[derive(Debug, Copy, Clone, PartialEq)]
/// Define what physics phenomena the FourVector<T> should represent
pub enum FourVectorType {
    Event,
    EventInterval,
    Velocity,
    EnrgImpulsion,
}

#[derive(Debug, Copy, Clone)]
/// Elementary data type to represent all special relativity phenomena\
/// Should use as T type value that can take floating point value like f64 or decimal from external crate
pub struct FourVector<T> {
    /// Look at FourVectorType
    pub representation: FourVectorType,
    /// Space time coordinate
    pub coordinate: [T; 4],
    /// Standard basis, (+, −, −, −) signature if set true else (-, +, +, +)\
    /// /!\\ Use the same for every FourVector
    pub signature: bool,
}

/// Return space time coordinate
pub trait Getters<T> {
    fn w(&self) -> T;

    fn x(&self) -> T;

    fn y(&self) -> T;

    fn z(&self) -> T;
}

/// Set of initialisation function
pub trait Init<T> {
    /// Basic init function
    fn new(rep: FourVectorType, w: T, x: T, y: T, z: T, sig: bool) -> Self;
}

/// Required methodes to perform crate calculation
pub trait LinearAlgebra<T> {
    /// Perform dot product betwin two FourVector<T>, return a scalar
    fn dot(&self, other: &Self) -> T;

    /// Return FourVector lenght.\
    /// Followin this schematic: sqrt(w * w' - x * x' - y * y' - z * z') for
    /// Standard basis, (+, −, −, −) signature\
    /// and sqrt(- w * w' + x * x' + y * y' + z * z') for
    /// Standard basis, (-, +, +, +) signature
    fn norm(&self) -> T;

    /// Return the normelized vector by performing vec / vec.norm()\
    /// You may not wan't use this methode if you use integer value as T type
    fn normelized(&self) -> FourVector<T>;
}

impl<T> Getters<T> for FourVector<T>
where
    T: Copy,
{
    fn w(&self) -> T {
        self.coordinate[0]
    }

    fn x(&self) -> T {
        self.coordinate[1]
    }

    fn y(&self) -> T {
        self.coordinate[2]
    }

    fn z(&self) -> T {
        self.coordinate[3]
    }
}

impl<T> Init<T> for FourVector<T> {
    fn new(rep: FourVectorType, w: T, x: T, y: T, z: T, sig: bool) -> Self {
        Self {
            representation: rep,
            coordinate: [w, x, y, z],
            signature: sig,
        }
    }
}

impl<T> LinearAlgebra<T> for FourVector<T>
where
    T: Mul<Output = T> + Neg<Output = T> + SubAssign
        + Copy
        + Sqrt<T>
{
    /// FourVector must have same signature
    fn dot(&self, other: &Self) -> T {
        if self.signature != other.signature {
            panic!("Cannot process norm of two quadrivector with different representation")
        }
        let mut out: T = self.w() * other.w();
        for i in 1..4 {
            out -= self.coordinate[i] * other.coordinate[i];
        }
        if self.signature {
            out
        } else {
            -out
        }
    }

    fn norm(&self) -> T {
        match Sqrt::norm_sqrt(&self.dot(self)) {
            Ok(value) => value,
            Err(_) => panic!("Invalide value"),
        }
    }

    fn normelized(&self) -> FourVector<T> {
        *self
    }
}

/// Use this insteed of PartialEq or Eq to compare two FourVector<T>
pub fn compare<T>(vector1: FourVector<T>, vector2: FourVector<T>) -> Result<bool, String>
where
    T: PartialEq,
{
    if vector1.representation != vector2.representation {
        return Err(String::from(
            "Cannot compare two different physic phenomena.",
        ));
    }
    Ok(vector1.coordinate[0] == vector2.coordinate[0]
        && vector1.coordinate[1] == vector2.coordinate[1]
        && vector1.coordinate[2] == vector2.coordinate[2]
        && vector1.coordinate[3] == vector2.coordinate[3])
}

impl<T> Div<T> for &FourVector<T>
where
    T: Div<T, Output = T>
        + Copy,
{
    type Output = FourVector<T>;

    fn div(self, other: T) -> Self::Output {
        Self::Output {
            representation: self.representation,
            coordinate: [
                self.coordinate[0] / other,
                self.coordinate[1] / other,
                self.coordinate[2] / other,
                self.coordinate[3] / other,
            ],
            signature: self.signature,
        }
    }
}

impl<T> Index<usize> for FourVector<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            // TODO: edit code when `https://github.com/rust-lang/rust/issues/37854` issues corrected
            0 | 1 | 2 | 3 => &self.coordinate[i],
            _ => panic!("Index {} out of range", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getters() {
        let quad: FourVector<u8> = FourVector::new(FourVectorType::Event, 12, 45, 2, 0, true);
        assert_eq!(quad.w(), 12);
        assert_ne!(quad.x(), 23);
        assert_eq!(quad.y(), quad[2]);
        assert_ne!(quad[3], 1);
    }

    #[test]
    #[should_panic]
    fn crash_test() {
        let quad: FourVector<i32> = FourVector::new(FourVectorType::Event, 21, 41, -12, -67, true);
        quad[5];
    }
}
