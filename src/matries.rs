use std::marker::PhantomData;
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const R: usize, const C: usize> {
    values: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(arrays: [[T; C]; R]) -> Self {
        Matrix { values: arrays }
    }
}

pub struct DynamicMatrix<T> {
    values: Vec<T>,
}

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0][index.1]
    }
}

pub trait MatrixSolver {}

pub enum SparesMatrixType {
    CSC = 0,
}

pub struct SparseMatrix<T, const U: usize> {
    values: Vec<T>,
    col_index: Vec<usize>,
    row_index: Vec<usize>,
}

pub type CSC<T> = SparseMatrix<T, 0>;
pub type CSR<T> = SparseMatrix<T, 1>;
pub type COO<T> = SparseMatrix<T, 2>;

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
