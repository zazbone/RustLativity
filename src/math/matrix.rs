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


impl<T> Index<(usize, usize)> for LorentzMatrix<T> {
    type Output = T;

    fn index(&self, lc: (usize, usize)) -> &Self::Output {
            &self.0[lc.0][lc.1]
    }
}
