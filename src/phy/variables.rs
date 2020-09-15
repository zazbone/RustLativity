mod speed;

use crate::utils::constants::Light;
use speed::Speed;

use std::op::Div;

pub struct LorentzFactor<T> {
    pub beta: T
}

pub struct BetaFactor<T> {
    // TODO
}


trait Init<T> {
    fn new_from_speed(speed: Speed) -> Self;
}


impl<T> Init<T> for LorentzFactor
where T: Div<Output=T>
        + Light
{
    fn new_from_speed(speed: Speed) -> Self {
        Self {
            beta: speed / T::light_speed()
        }
    }
}
