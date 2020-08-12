use crate::algebra::vec3::Vec3;


use std::ops;


type Event = QuadVector;

type Speed = QuadVector;

type EnergyImpulsion = QuadVector;

#[derive(Debug, Copy, Clone)]
pub struct QuadVector {
    pub w: f64,
    pub r: Vec3
}


impl QuadVector {
    pub fn new() -> Self {
        Self {w: 0.0, r: Vec3::new()}
    }
}


impl ops::Index<usize> for QuadVector {
    type Output = f64;
    // Why &f64 needed ??
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.w,
            1 | 2 | 3 => &self.r[index],
            _ => panic!("Index out of range.")
        }
    }
}

impl ops::IndexMut<usize> for QuadVector {
    // I think i understand ... But not sure
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.w,
            1 | 2 | 3 => &mut self.r[index],
            _ => panic!("Index out of range.")
        }
    }
}
