use num::traits::{Zero};

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
}
