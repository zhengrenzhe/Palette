use std::ops::{Add, Sub};

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn mul(&self, num: f64) -> Vec3 {
        Vec3::new(self.x * num, self.y * num, self.z * num)
    }
}

/// vec3 + vec3
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/// vec3 - vec3
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        let r = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(r.x, 5.0_f64);
        assert_eq!(r.y, 7.0_f64);
        assert_eq!(r.z, 9.0_f64);
    }

    #[test]
    fn test_vec3_sub() {
        let r = Vec3::new(8.0, 7.0, 6.0) - Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(r.x, 7.0_f64);
        assert_eq!(r.y, 5.0_f64);
        assert_eq!(r.z, 3.0_f64);
    }

    #[test]
    fn test_vec3_mul() {
        let r = Vec3::new(8.0, 7.0, 6.0).mul(4.0);
        assert_eq!(r.x, 32.0_f64);
        assert_eq!(r.y, 28.0_f64);
        assert_eq!(r.z, 24.0_f64);
    }
}
