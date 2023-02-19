use derive_more::Constructor;
use crate::{vector::{Direction, Point, Color}, vec3};

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Ray {
    point: Point,
    direction: Direction,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
		self.point + self.direction * t
	}

    pub fn color(ray: &Ray) -> Color {
        vec3!(0, 1.0, 0)
    }
}
