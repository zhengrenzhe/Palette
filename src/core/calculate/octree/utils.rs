use crate::core::calculate::octree::point::Point;
use crate::core::calculate::octree::Octree;

impl Octree {
    fn walk(node: &Octree, f: &fn(octree: &Point)) {
        if node.point.is_some() {
            f(&node.point.unwrap());
        }
        return (&node.children).into_iter().for_each(|c| {
            if c.is_some() {
                Octree::walk(&c.as_ref().unwrap(), f);
            }
        });
    }

    pub fn for_each(&self, f: fn(octree: &Point)) {
        Octree::walk(&self, &f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::calculate::octree::point::Point;
    use crate::core::calculate::octree::vec3::Vec3;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_octree() {
        // center is (0, 0, 0), physical dimension is 100x100x100
        let mut tree = Octree::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(50.0, 50.0, 50.0));

        let mut rng = thread_rng();
        for index in 0..2 {
            let x = rng.gen_range(-50.0..50.0);
            let y = rng.gen_range(-50.0..50.0);
            let z = rng.gen_range(-50.0..50.0);
            let point = Point::new(Vec3::new(x, y, z), index);
            tree.insert(point);
        }

        tree.for_each(|c| println!("{:?}", c));
    }
}
