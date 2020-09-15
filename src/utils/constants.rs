pub trait Light<T> {
    /// Return speed of light value (in m.s^-1) for generic type T
    /// RustLativity implemented for rust primitiv type (f32, f64, u32-128, i32-128),
    fn light_speed() -> T;
}

impl Light<f32> for f32 {
    fn light_speed() -> f32 {
        299792458.0f32
    }
}

impl Light<f64> for f64 {
    fn light_speed() -> f64 {
        299792458.0f64
    }
}

impl Light<u32> for u32 {
    fn light_speed() -> u32 {
        299792458u32
    }
}

impl Light<u64> for u64 {
    fn light_speed() -> u64 {
        299792458u64
    }
}

impl Light<u128> for u128 {
    fn light_speed() -> u128 {
        299792458u128
    }
}

impl Light<i32> for i32 {
    fn light_speed() -> i32 {
        299792458i32
    }
}

impl Light<i64> for i64 {
    fn light_speed() -> i64 {
        299792458i64
    }
}

impl Light<i128> for i128 {
    fn light_speed() -> i128 {
        299792458i128
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn light_speed() {
        assert_approx_eq!(f32::light_speed(), 299792458.0f32, 10e-5);
        assert_approx_eq!(f64::light_speed(), 299792458.0f64, 10e-5);
        assert_eq!(u32::light_speed(), 299792458u32);
        assert_eq!(u64::light_speed(), 299792458u64);
        assert_eq!(u128::light_speed(), 299792458u128);
        assert_eq!(i32::light_speed(), 299792458i32);
        assert_eq!(i64::light_speed(), 299792458i64);
        assert_eq!(i128::light_speed(), 299792458i128);
    }
}
