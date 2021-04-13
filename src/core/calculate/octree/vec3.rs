use std::ops::{Add, Mul, Sub};

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

/// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// Vec3 * n
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_vec3_add() {
        let r = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
        assert!(approx_eq!(f64, r.x, 5.0));
        assert!(approx_eq!(f64, r.y, 7.0));
        assert!(approx_eq!(f64, r.z, 9.0));
    }

    #[test]
    fn test_vec3_sub() {
        let r = Vec3::new(8.0, 7.0, 6.0) - Vec3::new(1.0, 2.0, 3.0);
        assert!(approx_eq!(f64, r.x, 7.0));
        assert!(approx_eq!(f64, r.y, 5.0));
        assert!(approx_eq!(f64, r.z, 3.0));
    }

    #[test]
    fn test_vec3_mul() {
        let r = Vec3::new(8.0, 7.0, 6.0) * 4.0;
        assert!(approx_eq!(f64, r.x, 32.0));
        assert!(approx_eq!(f64, r.y, 28.0));
        assert!(approx_eq!(f64, r.z, 24.0));
    }
}
