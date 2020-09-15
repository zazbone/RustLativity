use std::ops::{Mul, Add, Div};
use crate::utils::traits::Sqrt;


/// Use only for referential speed, use SpeedQuedrivector for relativ speed
pub struct Speed<T> {
    pub direction: [T; 3],
    pub lenght: T
}


/// Should use as T type value that can take floating point value like f64 or decimal from external crate
pub trait Init<T> {
    fn new(x: T, y: T, z: T) -> Self;
}


impl<T> Init<T> for Speed<T> where
    T: Mul<Output=T> + Add<Output=T> + Div<Output=T> + Sqrt<T>
        + Copy
{
    fn new(x: T, y: T, z: T) -> Self {
        let norm: T = T::generic_sqrt(&(x * x + y * y + z * z)).expect("Unexpected behavior");
        Self {
            direction: [
                x / norm,
                y / norm,
                z / norm
            ],
            lenght: norm
        }
    }
}
