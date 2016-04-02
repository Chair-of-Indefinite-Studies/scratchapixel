use std::ops::{Mul,Add,Sub};
use num::traits::{Zero,One,Float};

#[derive(PartialEq,Debug)]
pub struct Vec3<T> where T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Zero + Copy + Clone {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T> Vec3<T> where T : Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Zero + Copy + Clone {
    pub fn new(xx: T, yy: T, zz: T) -> Vec3<T> {
        Vec3 { x: xx, y: yy, z: zz }
    }

    pub fn zero() -> Vec3<T> {
        Vec3::new(T::zero(), T::zero(), T::zero())
    }

    pub fn diagonal(xx: T) -> Vec3<T> {
        Vec3::new(xx, xx, xx)
    }

    #[allow(non_snake_case)]
    pub fn dotProduct(self, rhs: Vec3<T>) -> T {
         self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[allow(non_snake_case)]
    pub fn crossProduct(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
            )
    }

    pub fn norm(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

}

impl <T> Add<Vec3<T>> for Vec3<T> where T : Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Zero + Copy + Clone {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            )
    }
}

impl <T> Sub<Vec3<T>> for Vec3<T> where T : Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Zero + Copy + Clone {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            )
    }
}

impl <T> Mul<T> for Vec3<T> where T : Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Zero + Copy + Clone {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            )
    }
}

impl <T> Vec3<T> where T: Mul<T, Output=T> + One + Zero + Float + Clone + Copy {
    #[allow(non_snake_case)]
    pub fn sphericalToCartesian(theta: T, phi: T) -> Vec3<T> {
        Vec3::new(phi.cos() * theta.sin(), phi.sin() * theta.sin(), theta.cos())
    }

    fn clamp(value: T, low: T, high: T) -> T {
        if value < low {
            return low
        }
        if value > high {
            return high
        }
        value
    }

    #[allow(non_snake_case)]
    pub fn sphericalTheta(v: &Vec3<T>) -> T {
       Vec3::clamp(v.z, -T::one(), T::one()).acos()
    }

    #[allow(non_snake_case)]
    pub fn sphericalPhi(v: &Vec3<T>) -> T {
        v.y.atan2(v.x) // TODO should be in range [0..2PI)
    }

    #[allow(non_snake_case)]
    pub fn cosTheta(v: &Vec3<T>) -> T {
        v.z
    }

    #[allow(non_snake_case)]
    pub fn sinTheta2(v: &Vec3<T>) -> T {
        let positive_result = T::one() - Vec3::cosTheta(&v) * Vec3::cosTheta(&v);
        if positive_result >= T::zero() {
            positive_result
        } else {
            T::zero()
        }
    }

    #[allow(non_snake_case)]
    pub fn sinTheta(v: &Vec3<T>) -> T {
        Vec3::sinTheta2(v).sqrt()
    }

    #[allow(non_snake_case)]
    pub fn cosPhi(v: &Vec3<T>) -> T {
        let sinTheta: T = Vec3::sinTheta(v);
        if sinTheta == T::zero() {
            T::one()
        } else {
            Vec3::clamp(v.x / sinTheta, -T::one(), T::one())
        }
    }

    #[allow(non_snake_case)]
    pub fn sinPhi(v: &Vec3<T>) -> T {
        let sinTheta: T = Vec3::sinTheta(v);
        if sinTheta == T::zero() {
            T::zero()
        } else {
            Vec3::clamp(v.y / sinTheta, -T::one(), T::one())
        }
    }

    pub fn length(& self) -> T {
        self.norm().sqrt()
    }

    pub fn normalize(&mut self) {
        let length: T = self.length();
        if length > T::zero() {
            let inverse_length = T::one() / length;

            self.x = self.x * inverse_length;
            self.y = self.y * inverse_length;
            self.z = self.z * inverse_length;
        }
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

        let dot_product: f64 = u.dotProduct(v);

        assert_eq!(dot_product, 3.0);
    }

    #[test]
    fn should_know_its_length() {
        let v: Vec3<f64> = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

        assert_eq!(v.length(), 0.0);
    }

    #[test]
    fn should_be_able_to_normalize() {
        let mut v: Vec3<f64> = Vec3::new(3.0, 0.0, 0.0);
        v.normalize();

        assert_eq!(v, Vec3 { x: 1.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn should_know_cross_product() {
        let v: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);
        let w: Vec3<f64> = Vec3::new(0.0, 1.0, 0.0);

        let product = v.crossProduct(w);

        assert_eq!(product, Vec3 { x: 0.0, y: 0.0, z: 1.0 });
    }

    #[test]
    fn should_form_vector_sum() {
        let v: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);
        let w: Vec3<f64> = Vec3::new(0.0, 1.0, 0.0);

        let sum = v + w;

        assert_eq!(sum, Vec3 { x: 1.0, y: 1.0, z: 0.0 });
    }

    #[test]
    fn should_form_vector_difference() {
        let v: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);
        let w: Vec3<f64> = Vec3::new(0.0, 1.0, 0.0);

        let sum = v - w;

        assert_eq!(sum, Vec3 { x: 1.0, y: -1.0, z: 0.0 });
    }

    #[test]
    fn should_form_scalar_multiplication() {
        let v: Vec3<f64> = Vec3::new(1.0, 0.0, 0.0);
        let c: f64 = 2.0;

        let scalar = v * c;

        assert_eq!(scalar, Vec3 { x: 2.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn should_know_how_to_convert_from_polar_coordinates() {
        let v: Vec3<f64> = Vec3::sphericalToCartesian(0.0, 0.0);

        assert_eq!(v, Vec3 { x: 0.0, y: 0.0, z: 1.0 });
    }
}
