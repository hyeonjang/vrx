pub enum SparesMatrixType {
    COO, 
    CSR,
    CRS,
}

pub struct SpareMatrix<T> {
    val: Vec<T>,
    col_index: Vec<usize>,
    row_index: Vec<usize>,
}