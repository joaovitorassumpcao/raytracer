use derive_more::Constructor;

use crate::{
    ray::Ray,
    vector::{Point, Vec3},
};

pub trait Object {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit>;
}

pub struct Hit {
    pub intersec: Point,
    pub normal: Vec3,
    pub t: f64,
}

#[derive(Debug, Constructor)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, (bound_start, bound_end): (f64, f64)) -> Option<Hit> {
        let v = ray.direction;
        let oc = ray.origin - self.center;
        let a = v.dot(&v);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let a2 = 2.0 * a;
        let mut root = (-b - discriminant_sqrt) / a2;
        if (bound_start..bound_end).contains(&root) {
            root = (-b + discriminant_sqrt) / a2;
            if (bound_start..bound_end).contains(&root) {
                return None;
            }
        }

        let intersec = ray.at(root);
        let normal = (intersec - self.center) / self.radius;

        Some( Hit {
            intersec,
            normal,
            t: root,
        })
    }
}
