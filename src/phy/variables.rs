use crate::utils::constants::Light;
use crate::phy::speed::Speed;

use std::ops::Div;

pub struct LorentzFactor<T> {
    pub beta: T
}

pub struct BetaFactor<T>(T);


trait Init<T> {
    fn new_from_speed(speed: Speed<T>) -> Self;
}


impl<T> Init<T> for LorentzFactor<T>
where T: Div<Output=T>
        + Light<T>
{
    fn new_from_speed(speed: Speed<T>) -> Self {
        Self {
            beta: speed.lenght / T::light_speed()
        }
    }
}
