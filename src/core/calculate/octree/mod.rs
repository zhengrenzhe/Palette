use crate::core::calculate::octree::point::Point;
use crate::core::calculate::octree::vec3::Vec3;
use std::mem::replace;

mod point;
mod vec3;

#[derive(Debug)]
pub struct Octree {
    center: Vec3,
    half: Vec3,
    point: Option<Point>,
    children: Vec<Option<Octree>>,
}

impl Octree {
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
    pub fn new(center: Vec3, half: Vec3) -> Octree {
        // payload default is None
        let payload = Option::None;
        // 8 children default is None
        // child: 0 1 2 3 4 5 6 7
        // x: - - - - + + + +
        // y: - - + + - - + +
        // z: - + - + - + - +
        let children: Vec<Option<Octree>> = (0..8).into_iter().map(|_| Option::None).collect();
        Octree {
            center,
            half,
            point: payload,
            children,
        }
    }

    pub fn insert(&mut self, point: Point) {
        if self.is_leaf() {
            if self.point.is_none() {
                // is leaf node, but has no payload for this node, so assign point to payload.
                self.point = Option::Some(point);
                return;
            } else {
                // is leaf node, already has payload, so split this node to 8 child nodes,
                // and insert old payload to next level

                // set current node payload to None
                let old_point = self.point.unwrap();
                self.point = Option::None;

                // split current node to 8 child nodes
                for i in 0..8 {
                    let mut new_center = self.center;
                    new_center.x = self.half.x * if i & 4 == 0 { -0.5 } else { 0.5 };
                    new_center.y = self.half.y * if i & 2 == 0 { -0.5 } else { 0.5 };
                    new_center.z = self.half.z * if i & 1 == 0 { -0.5 } else { 0.5 };

                    replace(
                        &mut self.children[i],
                        Option::Some(Octree::new(new_center, self.half * 0.5)),
                    );
                }

                // re-insert old point
                let old_point_will_insert_to = self.get_insert_child_index(&old_point.position);
                self.children
                    .get_mut(old_point_will_insert_to)
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .insert(old_point);

                // insert new point
                let new_point_will_insert_to = self.get_insert_child_index(&point.position);
                self.children
                    .get_mut(new_point_will_insert_to)
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .insert(point);
            }
        } else {
            let new_point_will_insert_to = self.get_insert_child_index(&point.position);
            self.children
                .get_mut(new_point_will_insert_to)
                .unwrap()
                .as_mut()
                .unwrap()
                .insert(point);
        }
    }

    /// if the first one is None, it indicates there are no children of this node, so this node
    /// is leaf node
    fn is_leaf(&self) -> bool {
        self.children.get(0).unwrap().is_none()
    }

    fn get_insert_child_index(&self, point: &Vec3) -> usize {
        let mut oct = 0;
        if point.x >= self.center.x {
            oct |= 4;
        }
        if point.y >= self.center.y {
            oct |= 2;
        }
        if point.z >= self.center.z {
            oct |= 1;
        }
        return oct;
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
        let mut tree = Octree::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(50.0, 50.0, 50.0));

        let mut rng = thread_rng();
        for index in 0..2 {
            let x = rng.gen_range(-50.0..50.0);
            let y = rng.gen_range(-50.0..50.0);
            let z = rng.gen_range(-50.0..50.0);
            let point = Point::new(Vec3::new(x, y, z), index);
            tree.insert(point);
        }

        println!("{:?}", tree);
    }
}
