use num::traits::{Zero,One};
use std::ops::{Mul,Add,Sub,Index};

#[derive(PartialEq,Debug)]
pub struct Matrix44<T> {
    pub m: [[T; 4]; 4],
}

impl <T> Matrix44<T> where T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Zero + One + Copy + Clone {
    pub fn zero() -> Matrix44<T> {
        Matrix44 { m: [
            [T::zero(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::zero()], ]}
    }

    pub fn diagonal(p: T, q: T, r: T, s: T) -> Matrix44<T> {
        Matrix44 { m: [
            [p, T::zero(), T::zero(), T::zero()],
            [T::zero(), q, T::zero(), T::zero()],
            [T::zero(), T::zero(), r, T::zero()],
            [T::zero(), T::zero(), T::zero(), s], ]}
    }

    pub fn new(mm: [[T; 4]; 4]) -> Matrix44<T> {
        Matrix44 { m : mm }
    }
}

impl <T> Index<usize> for Matrix44<T> where T: Zero {
    type Output = [T; 4];

    fn index(&self, index: usize) -> &[T; 4] {
       &(self.m[index])
    }
}

impl <T> Mul<Matrix44<T>> for Matrix44<T> where T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Zero + One + Copy + Clone {
    type Output = Matrix44<T>;

    fn mul(self, rhs: Matrix44<T>) -> Matrix44<T> {
        let mut result: [[T; 4]; 4] = [[T::zero(); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[i][0] * rhs[0][j] +
                    self[i][1] * rhs[1][j] +
                    self[i][2] * rhs[2][j] +
                    self[i][3] * rhs[3][j]
            }
        }
        Matrix44::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_initialized_as_zero_matrix() {
        let m: Matrix44<f64> = Matrix44::zero();

        assert_eq!(m, Matrix44::<f64> { m: [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            ]});
    }

    #[test]
    fn should_be_initialized_by_coefficients() {
        let m: Matrix44<f64> = Matrix44::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 3.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            ]);

        assert_eq!(m, Matrix44::<f64> { m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 3.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            ]});
    }

    #[test]
    fn should_index_rows() {
        let m: Matrix44<f64> = Matrix44::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 3.0, 4.0, 5.0],
            [3.0, 4.0, 5.0, 6.0],
            [4.0, 5.0, 6.0, 7.0], ]);

        let row = m[1];

        assert_eq!(row, [2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn should_matrix_multiply() {
        let m: Matrix44<f64> = Matrix44::diagonal(1.0, 2.0, 3.0, 4.0);
        let n: Matrix44<f64> = Matrix44::diagonal(4.0, 3.0, 2.0, 1.0);

        let product: Matrix44<f64> = m * n;

        assert_eq!(product, Matrix44::<f64> { m: [
            [4.0, 0.0, 0.0, 0.0],
            [0.0, 6.0, 0.0, 0.0],
            [0.0, 0.0, 6.0, 0.0],
            [0.0, 0.0, 0.0, 4.0],
        ]})
    }
}
