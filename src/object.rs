use std::ops::Mul;

use derive_more::Constructor;

use crate::{ray::Ray, vector::Point, vector::Vec3};

#[derive(Debug, Constructor)]
struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: Ray) -> bool {
        let v: Vec3 = ray.direction;
        let oc = ray.origin - self.center;
        let a = v.dot(&v);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;
        discriminant > 0.0
    }
}
