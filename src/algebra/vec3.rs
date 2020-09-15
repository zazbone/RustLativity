use std::option::Option;
use std::ops::{Add, Div};


struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

trait Init<T> {
    fn new(x: T, y:T, z: T) -> Self;

    fn new_normed(x: T, y: T, z: T, norm: Option<T>) -> Self;
}


impl<T> Init<T> for Vec3<T> where
    T: Add<T, Output=T> + Div<T, Output=T> // operation
        + Copy
{
    fn new(x: T, y:T, z: T) -> Self {
        Vec3{x, y, z}
    }

    fn new_normed(mut x: T, mut y: T, mut z: T, norm: Option<T>) -> Self {
        let lenght: T = x + y + z;
        match norm {
            Some(value) => {
                x = x / (lenght / value);
                y = y / (lenght / value);
                z = z / (lenght / value);
            }
            None => {
                x = x / lenght;
                y = y / lenght;
                z = z / lenght;
            }
        }
        Vec3 {x, y, z}
    }
}
