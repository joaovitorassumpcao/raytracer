use crate::vector::{Point, Vec3};

struct Camera {
	origin: Point,		// Camera a ray origin
	tl_corner: Point,	// Top Left Corner
	h_vec: Vec3,		// Viewport's horizontal vector
	v_vec: Vec3,		// Viewport's vertical vector
}