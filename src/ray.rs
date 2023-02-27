#![allow(dead_code)]
use crate::{
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
        // Calculate the unit vector of the ray direction
        // and use it to calculate the t value
        // t = 0.5 * (unit_direction.y + 1.0)
        let t = 0.5 * (ray.direction.normalize().y + 1.0);

        let sphere = crate::object::Sphere::new(vec3!(0, 0, -1), 0.5);
        if sphere.hit(ray) {
            return vec3!(1, 0, 0);
        }

        // Linearly interpolate between white and blue
        Vec3::lerp(vec3!(0.5, 0.7, 1.0), vec3!(1), t)
    }
}
