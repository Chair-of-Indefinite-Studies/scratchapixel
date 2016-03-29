pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn should_know_its_length() {
        let v: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

        assert_eq!(v.length(), 0.0);
    }
}
