use std::marker::Sized;

/// Square root generic definition
/// Require for T type in trait crate::algebra::quad::LinearAlgebra<T>
pub trait Sqrt<T>
where
    T: Sized,
{
    /// Mathematicly correct sqrt
    /// Must return Err(()) if sqrt(self) value is not mathematicly defined
    /// Look for norm_sqrt is self may take negativ value
    fn generic_sqrt(&self) -> Result<T, ()>;

    /// Return the square root of self mathematics norm
    fn norm_sqrt(&self) -> Result<T, ()>;
}

/// Implementation of Sqrt<T> for f64 type
/// Mostly a warper of f64.sqrt()
impl Sqrt<f64> for f64 {
    /// Insted of returning NaN for invalid f64.sqrt() return Error.
    fn generic_sqrt(&self) -> Result<Self, ()> {
        let value: f64 = *self;
        if value < 0.0 {
            Err(())
        } else {
            Ok(f64::sqrt(value))
        }
    }

    fn norm_sqrt(&self) -> Result<Self, ()> {
        let value: f64 = *self;
        if value < 0.0 {
            Ok(f64::sqrt(-value))
        } else {
            Ok(f64::sqrt(value))
        }
    }
}

/// Implementation of Sqrt<T> for f32 type
/// Mostly a warper of f32.sqrt()
impl Sqrt<f32> for f32 {
    /// Insted of returning NaN for invalid f32.sqrt return Error.
    fn generic_sqrt(&self) -> Result<Self, ()> {
        let value: f32 = *self;
        if value < 0.0 {
            Err(())
        } else {
            Ok(f32::sqrt(value))
        }
    }

    fn norm_sqrt(&self) -> Result<Self, ()> {
        let value: f32 = *self;
        if value < 0.0 {
            Ok(f32::sqrt(-value))
        } else {
            Ok(f32::sqrt(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn float_sqrt() {
        let x: f64 = 2.0;
        let sqrt_x = match x.generic_sqrt() {
            Ok(value) => value,
            Err(_) => panic!("Invalid value for sqrt"),
        };
        assert_approx_eq!(sqrt_x, 1.414f64, 10e-2f64);

        let x: f32 = 4233.39024;
        let sqrt_x = match x.generic_sqrt() {
            Ok(value) => value,
            Err(_) => panic!("Invalid value for sqrt"),
        };
        assert_approx_eq!(sqrt_x, 65.0645f32, 10e-2f32);
    }

    #[test]
    #[should_panic]
    fn float_sqrt_err() {
        let y: f64 = -1.0;
        let _sqrt_y = match y.generic_sqrt() {
            Ok(value) => value,
            Err(_) => panic!("Invalid value for sqrt"),
        };

        let y: f32 = -1.0;
        let _sqrt_y = match y.generic_sqrt() {
            Ok(value) => value,
            Err(_) => panic!("Invalid value for sqrt"),
        };
    }

    #[test]
    fn float_norm_sqrt() {
        let x: f64 = -2.0;
        let sqrt_x = match x.norm_sqrt() {
            Ok(value) => value,
            Err(_) => panic!("Un-espected error"),
        };
        assert_approx_eq!(sqrt_x, 1.414f64, 10e-2f64);

        let x: f32 = -4233.39024;
        let sqrt_x = match x.norm_sqrt() {
            Ok(value) => value,
            Err(_) => panic!("Un-espected error"),
        };
        assert_approx_eq!(sqrt_x, 65.0645f32, 10e-2f32);
    }
}
