use crate::{
    vec3,
    vector::{Color, Direction, Point, Vec3},
};
use derive_more::Constructor;

#[derive(Debug, PartialEq, PartialOrd, Clone, Constructor)]
pub struct Ray {
    origin: Point,
    direction: Direction,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn color(ray: &Ray) -> Color {
        let direction = ray.direction.normalize();
        let t = 0.5 * (direction.normalize().y + 1.0);

        Vec3::lerp(vec3!(1.0), vec3!(0.5, 0.7, 1), t)
    }
}
