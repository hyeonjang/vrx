mod matries;
use matries::*;

fn main() {
    let m = Matrix::<f32, 3, 3>::new([[1.0, 1.0, 2.0], [2.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    m.cholesky();
}
