#![allow(dead_code)]

use derive_more::Constructor;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    material::{Material, Reflection},
    ray::Ray,
    vector::{Point, Vec3},
};

pub trait Object {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit>;
}

#[derive(Debug)]
pub struct Hit {
    pub intersec: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub reflection: Option<Reflection>,
}

#[derive(Debug, Clone, Constructor)]
pub struct Sphere<T: Material> {
    pub center: Point,
    pub radius: f64,
    pub material: T,
}

impl<T> Object for Sphere<T>
where
    T: Material,
{
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        let v = ray.direction;
        let oc = ray.origin - self.center;
        let a = v.dot(&v);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powf(2.0);

        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let bound_range = bounds.0..=bounds.1;
        let mut root = (-b - discriminant.sqrt()) / (2.0 * a);
        match !(bound_range).contains(&root) {
            true => {
                root = (-b + discriminant.sqrt()) / (2.0 * a);
                if !(bound_range).contains(&root) {
                    return None;
                }
            }
            false => (),
        }

        let intersec = ray.at(root);
        let normal = (intersec - self.center) / self.radius;

        let (front_face, normal) = match ray.direction.dot(&normal) {
            x if x > 0.0 => (false, -normal),
            _ => (true, normal),
        };

        let hit = Hit {
            intersec,
            normal,
            t: root,
            front_face,
            reflection: None,
        };

        hit.reflection = self.material.scatter(ray, &hit);

        Some(hit)
    }
}

pub type Scene = Vec<Box<dyn Object + Sync>>;

impl Object for Scene {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit> {
        self.par_iter()
            .filter_map(|x| x.hit(ray, bounds))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
