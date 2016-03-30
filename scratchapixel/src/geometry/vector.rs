use std::ops::{Mul,Add};

pub struct Vec3<T: Mul<T, Output = T> + Add<T, Output = T> + Copy + Clone> {
    x: T,
    y: T,
    z: T,
}

impl <T: Mul<T, Output = T> + Add<T, Output = T> + Copy + Clone> Vec3<T> {
    pub fn norm(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn should_know_its_norm() {
        let v: Vec3<f64> = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

        assert_eq!(v.norm(), 0.0);
    }
}
