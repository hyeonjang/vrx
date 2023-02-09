use std::marker::PhantomData;
use std::ops::Index;

pub enum SparesMatrixType {
    CSC = 0,
}

pub trait SparesMatrixTypeOp {}

pub struct Storage {}

pub struct SparseMatrix<T, U: SparesMatrixTypeOp> {
    val: Vec<T>,
    col_index: Vec<usize>,
    row_index: Vec<usize>,
    _phantom: PhantomData<U>,
}

impl<T, U> Index<(usize, usize)> for SparseMatrix<T, U> where U: SparesMatrixTypeOp {
    type Output = T;

    fn index(&self, row_col:(usize, usize)) -> &Self::Output {
        &self.val[self.col_index[row_col.0]]
    }
}

pub trait SparseSolver {
    fn factorize();
}

#[cfg(test)] 
mod tests {
    #[test]
    fn works() {
        println!("some");
    }
}