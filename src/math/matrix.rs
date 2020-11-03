use std::ops::{
    // Accessors
    Index, IndexMut
};


pub struct LorentzMatrix<T>(pub [[T; 4]; 4]);


impl<T> Index<usize> for LorentzMatrix<T> {
    type Output = [T; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for LorentzMatrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


impl<T> Index<(usize, usize)> for LorentzMatrix<T> {
    type Output = T;

    fn index(&self, lc: (usize, usize)) -> &Self::Output {
            &self.0[lc.0][lc.1]
    }
}

impl<T> IndexMut<(usize, usize)> for LorentzMatrix<T> {
    fn index_mut(&mut self, lc: (usize, usize)) -> &mut Self::Output {
        &mut self.0[lc.0][lc.1]
    }
}
