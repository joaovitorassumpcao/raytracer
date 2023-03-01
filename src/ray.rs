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
}

pub fn color(ray: &Ray, scene: &impl Object) -> Color {
    match scene.hit(ray, (0.001, f64::INFINITY)) {
        Some(hit) => (hit.normal + vec3!(1)) / 2.0,
        None => {
            // Calculate the unit vector of the ray direction
            let t = 0.5 * (ray.direction.normalize().y + 1.0);

            // Linearly interpolate between white and blue
            Vec3::lerp(vec3!(0.5, 0.7, 1.0), vec3!(1), t)
        }
    }
}
