use derive_more::Constructor;
use crate::vector::{Direction, Point};

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Ray {
    point: Point,
    direction: Direction,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
		self.point + self.direction * t
	}

    pub fn color() {
        todo!()
    }
}
