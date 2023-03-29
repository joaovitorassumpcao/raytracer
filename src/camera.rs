//      Copyright João Vitor Cunha Assumpção 2023.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

#[allow(dead_code)]

use crate::{
    ray::Ray,
    vec3,
    vector::{Point, Vec3},
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
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const FOCAL_LEN: f64 = 1.0;
        const VIEW_HEIGHT: f64 = 2.0;
        const VIEW_WIDTH: f64 = ASPECT_RATIO * VIEW_HEIGHT;
        let origin = vec3!(0);
        let h_vec = vec3!(VIEW_WIDTH, 0, 0);
        let v_vec = vec3!(0, -VIEW_HEIGHT, 0);
        let tl_corner: Point = origin - h_vec / 2.0 - v_vec / 2.0 - vec3!(0, 0, FOCAL_LEN);

        Self {
            origin,
            tl_corner,
            h_vec,
            v_vec,
        }
    }
}

impl Camera {
    pub fn new(origin: Vec3, ratio: f64, focal: f64, (height, width): (f64, f64)) -> Self {
        let h_vec = vec3!(width, 0, 0);
        let v_vec = vec3!(0, -height, 0);
        let tl_corner: Point = origin - h_vec / 2.0 - v_vec / 2.0 - vec3!(0, 0, focal);

        Self {
            origin,
            tl_corner,
            h_vec,
            v_vec,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let ray_dir: Vec3 = self.tl_corner + u * self.h_vec + v * self.v_vec - self.origin;
        Ray::new(self.origin, ray_dir)
    }
}
