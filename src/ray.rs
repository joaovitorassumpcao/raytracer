use crate::vector::{Direction, Point};

struct Ray {
    point: Point,
    direction: Direction,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
		self.point + self.direction * t
	}
}
