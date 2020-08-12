use std::ops;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Vec3 {
    pub fn new() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0}
    }
}


impl ops::Index<usize> for Vec3 {
    type Output = f64;
    // Why &f64 needed ??
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range.")
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    // I think i understant ... But nor sure
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range.")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acces() {
        let vec1: Vec3 = Vec3{x:34.2, y: -12.0, z: 0.9};
        let mut vec2: Vec3 = Vec3::new();
        vec2.x = 34.2;
        assert_eq!(vec1.x, vec2.x);
        assert_eq!(vec1.y, -12.0);
        assert_ne!(vec1.z, vec2.z);

        vec2[1] = 16.0;
        assert_eq!(vec1[0], vec2[0]);
        assert_eq!(vec2[1], 16.0);
        assert_ne!(vec1[2], vec2[2]);
    }
}
