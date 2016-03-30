use std::ops::{Mul,Add};
use num::traits::{Zero,Float};


#[derive(PartialEq,Debug)]
pub struct Vec3<T> where T: Mul<T, Output = T> + Add<T, Output = T> + Zero + Copy + Clone {
    x: T,
    y: T,
    z: T,
}

impl <T> Vec3<T> where T : Mul<T, Output = T> + Add<T, Output = T> + Zero + Copy + Clone {
    pub fn new(xx: T, yy: T, zz: T) -> Vec3<T> {
        Vec3 { x: xx, y: yy, z: zz }
    }

    pub fn zero() -> Vec3<T> {
        Vec3::new(T::zero(), T::zero(), T::zero())
    }

    pub fn diagonal(xx: T) -> Vec3<T> {
        Vec3::new(xx, xx, xx)
    }
}

impl <T> Mul<Vec3<T>> for Vec3<T> where T: Mul<T, Output = T> + Add<T, Output = T> + Zero + Copy + Clone {
    type Output = T;

    fn mul(self, rhs: Vec3<T>) -> T {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl <T> Vec3<T> where T: Mul<T, Output=T> + Add<T, Output=T> + Zero + Float + Clone + Copy {
    pub fn length(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn should_be_created_with_constructor() {
        let v: Vec3<f64> = Vec3::new(0.0, 1.0, 2.0);

        assert_eq!(v, Vec3 { x: 0.0, y: 1.0, z: 2.0 });
    }

    #[test]
    fn should_initialize_to_zero_vector() {
        let v: Vec3<f64> = Vec3::zero();

        assert_eq!(v, Vec3::<f64>{ x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn should_initialize_to_diagonal_vector() {
        let v: Vec3<f64> = Vec3::diagonal(1.0);

        assert_eq!(v, Vec3::<f64>{ x: 1.0, y: 1.0, z: 1.0 });
    }

    #[test]
    fn should_know_dot_product() {
        let u: Vec3<f64> = Vec3::diagonal(1.0);
        let v: Vec3<f64> = Vec3::new(0.0, 1.0, 2.0);

        let dot_product: f64 = u * v;

        assert_eq!(dot_product, 3.0);
    }

    #[test]
    fn should_know_its_length() {
        let v: Vec3<f64> = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

        assert_eq!(v.length(), 0.0);
    }
}
