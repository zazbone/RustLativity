use crate::algebra::quad::QuadVector;


use std::ops;


#[derive(Debug, Copy, Clone)]
struct Row([f64; 4]);

#[derive(Debug, Copy, Clone)]
struct LorentzMatrix([Row; 4]);


impl LorentzMatrix {
    const IDENTITY: Self = LorentzMatrix {0: [
        Row{0: [1.0, 0.0, 0.0, 0.0]},
        Row{0: [0.0, 1.0, 0.0, 0.0]},
        Row{0: [0.0, 0.0, 1.0, 0.0]},
        Row{0: [0.0, 0.0, 0.0, 1.0]}
    ]};


    pub fn dot(&self, vec: &QuadVector) -> QuadVector {
        let mut out = QuadVector::new();
        for line in 0..4 {
            for column in 0..4 {
                out[line] += vec[line] * self[line][column];
            }
        }
        out
    }
}


impl ops::Index<usize> for Row {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// Unused
impl ops::Mul<f64> for &Row {
    type Output = Row;

    fn mul(self, scalar: f64) -> Self::Output {
        let mut out: Row = self.clone();
        for i in 0..4 {
            out[i] *= scalar;
        }
        out
    }
}

// Unused
impl ops::MulAssign<f64> for Row {
    fn mul_assign(&mut self, scalar: f64) {
        for i in 0..4 {
            self[i] *= scalar;
        }
    }
}

impl ops::Index<usize> for LorentzMatrix {
    type Output = Row;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for LorentzMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
