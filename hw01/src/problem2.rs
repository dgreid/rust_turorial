pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let rows = mat1.len();
    let cols = mat2[0].len();
    let mut newmat = vec![vec![0.;cols]; rows];

    assert!(mat1[0].len() == mat2.len());

    for col in 0..cols {
        for row in 0..rows {
            let mut sum = 0.;
            for i in 0..mat2.len() {
                sum = sum + mat1[row][i] * mat2[i][col];
            }
            newmat[row][col] = sum;
        }
    }

    newmat
}
