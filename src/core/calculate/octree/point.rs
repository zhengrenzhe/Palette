use crate::core::calculate::octree::vec3::Vec3;

pub struct Point<T> {
    pub position: Vec3,
    pub payload: T,
}

impl<T> Point<T> {
    pub fn new(position: Vec3, payload: T) -> Point<T> {
        Point { position, payload }
    }

    pub fn update_position(&mut self, position: Vec3) {
        self.position = position
    }

    pub fn update_payload(&mut self, payload: T) {
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
