use crate::math::vector::dot_product;

type MatrixNd<const N: usize> = [[f64; N]; N];

pub type Matrix2d = MatrixNd<2>;
pub type Matrix3d = MatrixNd<3>;
pub type Matrix4d = MatrixNd<4>;

pub fn add<const N: usize>(lhs: &MatrixNd<N>, rhs: &MatrixNd<N>) -> MatrixNd<N> {
    let mut result = [[0.; N]; N];
    for i in 0..N {
        for j in 0..N {
            result[i][j] = lhs[i][j] + rhs[i][j];
        }
    }
    result
}

pub fn sub<const N: usize>(lhs: &MatrixNd<N>, rhs: &MatrixNd<N>) -> MatrixNd<N> {
    let mut result = [[0.; N]; N];
    for i in 0..N {
        for j in 0..N {
            result[i][j] = lhs[i][j] - rhs[i][j];
        }
    }
    result
}

#[allow(clippy::needless_range_loop)]
pub fn mul<const N: usize>(lhs: &MatrixNd<N>, rhs: &MatrixNd<N>) -> MatrixNd<N> {
    let mut result = [[0.; N]; N];
    for i in 0..N {
        for j in 0..N {
            let mut elem: f64 = 0.;
            for k in 0..N {
                elem += lhs[i][k] * rhs[k][j];
            }
            result[i][j] = elem;
        }
    }
    result
}

#[allow(clippy::needless_range_loop)]
pub fn transpose<const N: usize>(matrix: &MatrixNd<N>) -> MatrixNd<N> {
    let mut result = [[0.; N]; N];
    for i in 0..N {
        for j in 0..N {
            result[i][j] = matrix[j][i];
        }
    }
    result
}

pub fn invert<const N: usize>(matrix: &MatrixNd<N>) -> MatrixNd<N> {
    transpose(&invert_transpose(matrix))
}

#[allow(clippy::needless_range_loop)]
/// Invert matrix via adjoint method, and use transpose to ease determinant base case.
fn invert_transpose<const N: usize>(matrix: &MatrixNd<N>) -> MatrixNd<N> {
    let vector_matrix: Vec<Vec<f64>> = (0..matrix.len())
        .map(|i| (0..matrix.len()).map(|j| matrix[i][j]).collect())
        .collect();

    let mut adjugate_transpose = [[0.; N]; N];
    for i in 0..N {
        for j in 0..N {
            adjugate_transpose[i][j] = cofactor(&vector_matrix, i, j);
        }
    }

    div(
        &adjugate_transpose,
        dot_product(&adjugate_transpose[0], &matrix[0]),
    )
}

fn cofactor(matrix: &[Vec<f64>], row: usize, column: usize) -> f64 {
    let mut submatrix: Vec<Vec<f64>> = vec![vec![0.; matrix.len() - 1]; matrix.len() - 1];

    for i in 0..submatrix.len() {
        for j in 0..submatrix.len() {
            submatrix[i][j] = matrix[i + (i >= row) as usize][j + (j >= column) as usize]
        }
    }

    det(&submatrix) * (if (row + column) % 2 == 0 { 1. } else { -1. })
}

fn div<const N: usize>(matrix: &MatrixNd<N>, factor: f64) -> MatrixNd<N> {
    let mut result = [[0.; N]; N];
    for i in 0..N {
        for j in 0..N {
            result[i][j] = matrix[i][j] / factor;
        }
    }

    result
}

fn det(matrix: &[Vec<f64>]) -> f64 {
    if matrix.len() == 1 {
        return matrix[0][0];
    }

    let mut result: f64 = 0.;
    for i in 0..matrix.len() {
        result += matrix[0][i] * cofactor(matrix, 0, i);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        {
            let lhs: Matrix2d = [[1., 2.], [3., 4.]];
            let rhs: Matrix2d = [[5., 6.], [7., 8.]];
            let exp: Matrix2d = [[19., 22.], [43., 50.]];
            assert_eq!(mul(&lhs, &rhs), exp);
        }
        {
            let lhs: Matrix3d = [[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];
            let rhs: Matrix3d = [[1., 2., 4.], [3., 5., 7.], [6., 8., 9.]];
            let exp: Matrix3d = [[25., 36., 45.], [55., 81., 105.], [85., 126., 165.]];
            assert_eq!(mul(&lhs, &rhs), exp);
        }
        {
            let lhs: Matrix4d = [
                [1., 2., 3., 4.],
                [5., 6., 7., 8.],
                [9., 10., 11., 12.],
                [13., 14., 15., 16.],
            ];
            let rhs: Matrix4d = [
                [1., 2., 4., 7.],
                [3., 5., 8., 11.],
                [6., 9., 12., 14.],
                [10., 13., 15., 16.],
            ];
            let exp: Matrix4d = [
                [65., 91., 116., 135.],
                [145., 207., 272., 327.],
                [225., 323., 428., 519.],
                [305., 439., 584., 711.],
            ];
            assert_eq!(mul(&lhs, &rhs), exp);
        }
    }

    #[test]
    fn test_transpose() {
        {
            let matrix: Matrix2d = [[1., 2.], [3., 4.]];
            let expected: Matrix2d = [[1., 3.], [2., 4.]];
            assert_eq!(transpose(&matrix), expected);
        }
        {
            let matrix: Matrix3d = [[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];
            let expected: Matrix3d = [[1., 4., 7.], [2., 5., 8.], [3., 6., 9.]];
            assert_eq!(transpose(&matrix), expected);
        }
        {
            let matrix: Matrix4d = [
                [1., 2., 3., 4.],
                [5., 6., 7., 8.],
                [9., 10., 11., 12.],
                [13., 14., 15., 16.],
            ];
            let expected: Matrix4d = [
                [1., 5., 9., 13.],
                [2., 6., 10., 14.],
                [3., 7., 11., 15.],
                [4., 8., 12., 16.],
            ];
            assert_eq!(transpose(&matrix), expected);
        }
    }

    #[test]
    fn test_determinant() {
        {
            let matrix = vec![vec![2., 1.], vec![1., 3.]];
            assert_eq!(det(&matrix), 5.);
        }
        {
            let matrix = vec![vec![1., 2., 3.], vec![3., 2., 1.], vec![2., 1., 3.]];
            assert_eq!(det(&matrix), -12.);
        }
    }

    #[test]
    fn test_cofactor() {
        let matrix = vec![vec![1., 2., 3.], vec![3., 2., 1.], vec![2., 1., 3.]];

        assert_eq!(cofactor(&matrix, 0, 0), 5.);
        assert_eq!(cofactor(&matrix, 0, 1), -7.);
        assert_eq!(cofactor(&matrix, 0, 2), -1.);
        assert_eq!(cofactor(&matrix, 1, 0), -3.);
        assert_eq!(cofactor(&matrix, 1, 1), -3.);
        assert_eq!(cofactor(&matrix, 1, 2), 3.);
        assert_eq!(cofactor(&matrix, 2, 0), -4.);
        assert_eq!(cofactor(&matrix, 2, 1), 8.);
        assert_eq!(cofactor(&matrix, 2, 2), -4.);
    }

    #[test]
    fn test_invert() {
        {
            let matrix: Matrix2d = [[4., 7.], [2., 6.]];
            let expected: Matrix2d = [[0.6, -0.7], [-0.2, 0.4]];
            assert_eq!(invert(&matrix), expected);
        }
        {
            let matrix: Matrix3d = [[1., 2., 3.], [3., 2., 1.], [2., 1., 3.]];
            let expected: Matrix3d = [
                [-5. / 12., 1. / 4., 1. / 3.],
                [7. / 12., 1. / 4., -2. / 3.],
                [1. / 12., -1. / 4., 1. / 3.],
            ];
            assert_eq!(invert(&matrix), expected);
        }
        {
            let matrix: Matrix4d = [
                [1., 1., 1., -1.],
                [1., 1., -1., 1.],
                [1., -1., 1., 1.],
                [-1., 1., 1., 1.],
            ];
            let expected: Matrix4d = [
                [1. / 4., 1. / 4., 1. / 4., -1. / 4.],
                [1. / 4., 1. / 4., -1. / 4., 1. / 4.],
                [1. / 4., -1. / 4., 1. / 4., 1. / 4.],
                [-1. / 4., 1. / 4., 1. / 4., 1. / 4.],
            ];
            assert_eq!(invert(&matrix), expected);
        }
    }
}
