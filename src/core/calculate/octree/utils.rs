use crate::core::calculate::octree::point::Point;
use crate::core::calculate::octree::Octree;

impl Octree {
    fn get_all_points(node: &Octree, vec: &mut Vec<Point>) {
        if node.point.is_some() {
            vec.push(node.point.unwrap());
        }
        return (&node.children).into_iter().for_each(|c| {
            if c.is_some() {
                return Octree::get_all_points(&c.as_ref().unwrap(), vec);
            }
        });
    }

    pub fn write(&self, path: &str) {
        let mut points: Vec<Point> = vec![];
        Octree::get_all_points(&self, &mut points);
        println!("{:?}", points);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::calculate::octree::point::Point;
    use crate::core::calculate::octree::vec3::Vec3;
    use crate::utils::msg_const::PROJ_DIR;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_get_all_points() {
        // let mut tree = Octree::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(50.0, 50.0, 50.0));
        // for index in 0..100000 {
        //     let point = Point::new(Vec3::new(1.0, 1.0, 1.0), 1);
        //     tree.insert(point);
        // }
        // let mut points: Vec<Point> = vec![];
        // Octree::get_all_points(&tree, &mut points);
        // assert_eq!(points.len(), 100000);
    }

    #[test]
    fn test_octree() {
        // center is (0, 0, 0), physical dimension is 100x100x100
        let mut tree = Octree::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(50.0, 50.0, 50.0));

        let mut rng = thread_rng();
        for index in 0..100000 {
            let x = rng.gen_range(-50.0..50.0);
            let y = rng.gen_range(-50.0..50.0);
            let z = rng.gen_range(-50.0..50.0);
            let point = Point::new(Vec3::new(x, y, z), index);
            tree.insert(point);
        }

        tree.write(&format!("{}/target/test_octree.octree", PROJ_DIR));
    }
}
