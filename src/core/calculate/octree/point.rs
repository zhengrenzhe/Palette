use crate::core::calculate::octree::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub position: Vec3,
    pub payload: i32,
}

impl Point {
    pub fn new(position: Vec3, payload: i32) -> Point {
        Point { position, payload }
    }

    pub fn update_position(&mut self, position: Vec3) {
        self.position = position
    }

    pub fn update_payload(&mut self, payload: i32) {
        self.payload = payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_point() {
        let mut p = Point::new(Vec3::new(1.0, 1.0, 1.0), 1);
        assert!(approx_eq!(f64, p.position.x, 1.0));
        assert!(approx_eq!(f64, p.position.y, 1.0));
        assert!(approx_eq!(f64, p.position.z, 1.0));
        assert_eq!(p.payload, 1);

        p.update_position(Vec3::new(2.0, 2.0, 2.0));
        p.update_payload(2);

        assert!(approx_eq!(f64, p.position.x, 2.0));
        assert!(approx_eq!(f64, p.position.y, 2.0));
        assert!(approx_eq!(f64, p.position.z, 2.0));
        assert_eq!(p.payload, 2);
    }
}
