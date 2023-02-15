use anyhow::*;
use matries::Factorizor;
use std::ffi::CString;
use std::ptr::{copy_nonoverlapping as memcpy, null};
use vkcholesky::vx::*;
use vkcholesky::*;

const COMP_SPV: &[u8] = include_bytes!("./shader/cholesky.spv");

mod matries;

fn main() -> Result<()> {
    let mat: matries::Matrix<f32, 4, 3> =
        matries::Matrix::new([[1.0, 2.0, 3.0, 4.0], [2.0, 3.0, 4.0, 5.0], [4.0, 5.0, 6.0, 7.0]]);
    mat.cholesky();
    Ok(())
}
