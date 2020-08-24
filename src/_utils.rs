use std::marker::Sized;


pub trait Sqrt<T> where
    T: Sized
{
    fn sqrt(&self) -> Result<T, ()>;

    /// Sqrt were T.n_sqrt() can take negativ value
    fn n_sqrt(&self) -> Result<T, ()>;
}


impl Sqrt<f64> for f64 {
    /// Insted of returning NaN for invalid sqrt return Error.
    fn sqrt(&self) -> Result<Self, ()> {
        let value: f64 = *self;
        if value < 0.0 {
            Err(())
        } else {
            Ok(f64::sqrt(value))
        }
    }

    fn n_sqrt(&self) -> Result<Self, ()> {
        let value: f64 = *self;
        if value < 0.0 {
            Ok(-f64::sqrt(-value))
        } else {
            Ok(f64::sqrt(value))
        }
    }
}

impl Sqrt<f32> for f32 {
    /// Insted of returning NaN for invalid sqrt return Error.
    fn sqrt(&self) -> Result<Self, ()> {
        let value: f32 = *self;
        if value < 0.0 {
            Err(())
        } else {
            Ok(f32::sqrt(value))
        }
    }

    fn n_sqrt(&self) -> Result<Self, ()> {
        let value: f32 = *self;
        if value < 0.0 {
            Ok(-f32::sqrt(-value))
        } else {
            Ok(f32::sqrt(value))
        }
    }
}
