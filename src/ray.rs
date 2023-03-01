#![allow(dead_code)]

use crate::{
    object::Object,
    vec3,
    vector::{Color, Direction, Point, Vec3},
};
use derive_more::Constructor;

#[derive(Debug, PartialEq, PartialOrd, Clone, Constructor)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn color(ray: &Ray) -> Color {
        let sphere = crate::object::Sphere::new(vec3!(0, 0, -1), 0.5);

        if let Some(hit) = sphere.hit(ray, (1.0, u64::MAX as f64)) {
            // return scaled surface normal
            let surface_normal = (ray.at(hit.t) - sphere.center).normalize();
            return surface_normal.map(|mut x| {
                x += 1.0;
                x / 2.0
            });
        }

        // Calculate the unit vector of the ray direction
        let t = 0.5 * (ray.direction.normalize().y + 1.0);

        // Linearly interpolate between white and blue
        Vec3::lerp(vec3!(0.5, 0.7, 1.0), vec3!(1), t)
    }
}
