use std::ops::{Mul,Add};
use num::traits::Zero;

#[derive(PartialEq,Debug)]
pub struct Vec3<T> where T: Mul<T, Output = T> + Add<T, Output = T> + Zero + Copy + Clone {
    x: T,
    y: T,
    z: T,
}

impl <T> Vec3<T> where T : Mul<T, Output = T> + Add<T, Output = T> + Zero + Copy + Clone {
    pub fn new() -> Vec3<T> {
        Vec3 { x: T::zero(), y: T::zero(), z: T::zero() }
    }

    pub fn norm(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn should_initialize_to_zero_vector() {
        let v: Vec3<f64> = Vec3::new();

        assert_eq!(v, Vec3::<f64>{ x: 0.0, y: 0.0, z: 0.0});
    }

    #[test]
    fn should_know_its_norm() {
        let v: Vec3<f64> = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

        assert_eq!(v.norm(), 0.0);
    }
}
