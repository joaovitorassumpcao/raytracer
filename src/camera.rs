use crate::{
    vec3,
    vector::{Point, Vec3}, ray::Ray,
};

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Point,    // Camera a ray origin
    tl_corner: Point, // Top Left Corner
    h_vec: Vec3,      // Viewport's horizontal vector
    v_vec: Vec3,      // Viewport's vertical vector
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let focal_len = 1.0;
        let view_height = 2.0;
        let view_width = aspect_ratio * view_height;
        let origin = vec3!(0);
        let h_vec = vec3!(view_width, 0, 0);
        let v_vec = vec3!(0, -view_height, 0);
        let tl_corner: Point = origin - h_vec / 2.0 - v_vec / 2.0 - vec3!(0, 0, focal_len);

        Self {
            origin,
            tl_corner,
            h_vec,
            v_vec,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let ray_dir: Vec3 = self.tl_corner + u * self.h_vec + v * self.v_vec - self.origin;
		Ray::new(self.origin, ray_dir)
    }
}
