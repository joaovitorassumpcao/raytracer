use derive_more::Constructor;

use crate::{ray::Ray, vector::Point};

#[derive(Debug, Constructor)]
struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn hit(ray: Ray) -> bool {
        todo!()
    }
}
