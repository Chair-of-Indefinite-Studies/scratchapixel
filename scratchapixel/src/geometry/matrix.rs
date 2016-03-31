use num::traits::{Zero};
use std::ops::Index;

#[derive(PartialEq,Debug)]
pub struct Matrix44<T> {
    pub m: [[T; 4]; 4],
}

impl <T> Matrix44<T> where T: Zero {
    pub fn zero() -> Matrix44<T> {
        Matrix44 { m: [
            [T::zero(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::zero()],
            [T::zero(), T::zero(), T::zero(), T::zero()], ]}
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
}
