use crate::utils::traits::Sqrt;


use std::ops::{
    Div,
    Mul,
    SubAssign
};
use std::result::Result;
use std::string::String;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum QuadVectorType {
    FourPosition,
    FourVelocity,
    FourMomentum
}


#[derive(Debug)]
pub struct QuadVector<T> {
    pub representation: QuadVectorType,
    pub coordinate: [T; 4]
}


pub trait Getters<T> {
    fn w(&self) -> T;

    fn x(&self) -> T;

    fn y(&self) -> T;

    fn z(&self) -> T;
}

pub trait Init<T> {
    fn new(rep: QuadVectorType, w: T, x: T, y: T, z: T) -> Self;
}

pub trait LinearAlgebra<T> {
    fn dot(&self, other: &Self) -> T;

    fn norm(&self) -> T;

    fn normelized(&self) -> T;
}


impl<T> Getters<T> for QuadVector<T> where
    T: Copy
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

impl<T> Init<T> for QuadVector<T>
{
    fn new(rep: QuadVectorType, w: T, x: T, y: T, z: T) -> Self {
        Self {
            representation: rep,
            coordinate: [w, x, y, z]
         }
    }
}

impl<T> LinearAlgebra<T> for QuadVector<T> where
    T: Mul<Output = T> + SubAssign + Sqrt<T> + Copy
{
    fn dot(&self, other: &Self) -> T {
        let mut out: T = self.w() * other.w();
        for i in 1..4 {
            out -= self.coordinate[i] * other.coordinate[i];
        }
        out
    }

    fn norm(&self) -> T {
        match Sqrt::norm_sqrt(&self.dot(self)) {
            Ok(value) => value,
            Err(_) => panic!("Invalide value")
        }
    }

    fn normelized(&self) -> T {
        self.w()
    }
}


pub fn compare<T>(vector1: QuadVector<T>, vector2: QuadVector<T>) -> Result<bool, String> where
    T: PartialEq
{
    if vector1.representation != vector2.representation {
        return Err(String::from(
            "Cannot compare two different physic phenomena."
        ))
    }
    Ok(
        vector1.coordinate[0] == vector2.coordinate[0] &&
        vector1.coordinate[1] == vector2.coordinate[1] &&
        vector1.coordinate[2] == vector2.coordinate[2] &&
        vector1.coordinate[3] == vector2.coordinate[3]
    )
}


impl<T> Div::<T> for &QuadVector<T> where
    T: Div<T, Output=T> + Copy
{
    type Output = QuadVector<T>;

    fn div(self, other: T) -> Self::Output {
        Self::Output {
            representation: self.representation,
            coordinate: [
                self.coordinate[0] / other,
                self.coordinate[1] / other,
                self.coordinate[2] / other,
                self.coordinate[3] / other
            ]
        }
    }
}
