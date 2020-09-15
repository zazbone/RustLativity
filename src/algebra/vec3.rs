use std::ops::{Add, Div, Mul};
use std::option::Option;

use crate::utils::traits::Sqrt;

pub struct Vec3<T> {
    direction: [T; 3],
    pub lenght: T,
}

trait Accessors<T> {
    fn x(&self) -> T;

    fn y(&self) -> T;

    fn z(&self) -> T;
}

trait Init<T> {
    fn new(x: T, y: T, z: T) -> Self;

    fn new_normed(x: T, y: T, z: T, norm: Option<T>) -> Self;
}

impl<T> Accessors<T> for Vec3<T>
where
    T: Mul<Output = T>
        + Copy,
{
    fn x(&self) -> T {
        self.direction[0] * self.lenght
    }

    fn y(&self) -> T {
        self.direction[1] * self.lenght
    }

    fn z(&self) -> T {
        self.direction[2] * self.lenght
    }
}

impl<T> Init<T> for Vec3<T>
where
    T: Add<T, Output = T> + Div<T, Output = T> + Mul<T, Output = T>
        + Copy
        + Sqrt<T>
{
    fn new(x: T, y: T, z: T) -> Self {
        let lenght: T = T::generic_sqrt(&(x * x + y * y + z * z)).expect("Unexpected behavior");
        Self {
            direction: [x / lenght, y / lenght, z / lenght],
            lenght,
        }
    }

    fn new_normed(mut x: T, mut y: T, mut z: T, norm: Option<T>) -> Self {
        let lenght: T;
        let len: T = T::generic_sqrt(&(x * x + y * y + z * z)).expect("Unexpected behavior");
        match norm {
            Some(value) => {
                x = x / (len / value);
                y = y / (len / value);
                z = z / (len / value);
                lenght = value;
            }
            None => {
                x = x / len;
                y = y / len;
                z = z / len;
                lenght = len;
            }
        }
        Self {
            direction: [x / lenght, y / lenght, z / lenght],
            lenght,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq;

    #[test]
    fn vec_init() {
        let vec: Vec3<f64> = Vec3::new(1., 0.7, 6.54);
        assert_approx_eq::assert_approx_eq!(vec.x(), 1., 10e-5);
        assert_approx_eq::assert_approx_eq!(vec.y(), 0.7, 10e-5);
        assert_approx_eq::assert_approx_eq!(vec.z(), 6.54, 10e-5);
    }
}
