pub trait Light<T> {
    /// Return speed of light value (in m.s^-1) for generic type T
    /// RustLativity implemented for rust primitiv type (f32, f64, u32-128, i32-128),
    fn speed() -> T;
}


impl Light<f32> for f32 {
    fn speed() -> f32 {
        299792458.0f32
    }
}

impl Light<f64> for f64 {
    fn speed() -> f64 {
        299792458.0f64
    }
}

impl Light<u32> for u32 {
    fn speed() -> u32 {
        299792458u32
    }
}

impl Light<u64> for u64 {
    fn speed() -> u64 {
        299792458u64
    }
}

impl Light<u128> for u128 {
    fn speed() -> u128 {
        299792458u128
    }
}

impl Light<i32> for i32 {
    fn speed() -> i32 {
        299792458i32
    }
}

impl Light<i64> for i64 {
    fn speed() -> i64 {
        299792458i64
    }
}

impl Light<i128> for i128 {
    fn speed() -> i128 {
        299792458i128
    }
}
