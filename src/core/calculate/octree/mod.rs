use crate::core::calculate::octree::point::Point;
use crate::core::calculate::octree::vec3::Vec3;

mod point;
mod vec3;

pub struct Octree<T> {
    center: Vec3,
    half: Vec3,
    payload: Option<Point<T>>,
    children: Vec<Option<Octree<T>>>,
}

impl<T> Octree<T> {
    /// create new octree
    ///
    /// # center: center position of octree
    /// # half: half width/height/depth of octree
    ///
    /// eg:
    ///    ```
    ///    let tree = Octree::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    ///    ```
    /// tree's center point is (0, 0, 0), physical dimension is 2x2x2
    pub fn new(center: Vec3, half: Vec3) -> Octree<T> {
        // payload default is None
        let payload = Option::None;
        // 8 children default is None
        let children: Vec<Option<Octree<T>>> = (0..8).into_iter().map(|_| Option::None).collect();
        Octree {
            center,
            half,
            payload,
            children,
        }
    }

    pub fn insert(&mut self, point: Point<T>) {
        if self.is_leaf() {
            if self.payload.is_none() {
                // is leaf node, but has no payload for this node, so assign point to payload.
                self.payload = Option::Some(point);
                return;
            } else {
                // is leaf node, already has payload, so split this node to 8 child nodes,
                //
            }
        }
    }

    /// if the first one is None, it indicates there are no children of this node, so this node
    /// is leaf node
    pub fn is_leaf(&self) -> bool {
        self.children.get(0).unwrap().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::calculate::octree::point::Point;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_octree() {
        // center is (0, 0, 0), physical dimension is 100x100x100
        let tree: Octree<i32> = Octree::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(50.0, 50.0, 50.0));

        let mut rng = thread_rng();
        for _ in 0..1 {
            let x = rng.gen_range(-50.0..50.0);
            let y = rng.gen_range(-50.0..50.0);
            let z = rng.gen_range(-50.0..50.0);
            let point = Point::new(Vec3::new(x, y, z), 1);
        }
    }
}
