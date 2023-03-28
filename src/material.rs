//      Copyright João Vitor Cunha Assumpção 2023.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use derive_more::Constructor;

use crate::{
    object::Hit,
    ray::Ray,
    vector::{Color, Vec3},
};

#[derive(Debug, Clone, Constructor)]
pub struct Reflection {
    pub ray: Ray,
    pub color_atten: Color,
}

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Metal {
    color: Color,
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Lambertian(Color);

impl Material for Lambertian {
    fn scatter(&self, _incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let mut direction: Vec3 = hit.normal + Vec3::rand_unitvec();

        if direction.is_zero() {
            direction = hit.normal;
        }

        let ray = Ray::new(hit.intersec, direction);
        Some(Reflection {
            ray,
            color_atten: self.0,
        })
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let reflection = reflect(incident_ray.direction, &hit.normal);
        let scattered = Ray::new(hit.intersec, reflection);

        match scattered.direction.dot(&hit.normal) > 0.0 {
            true => Some(Reflection {
                ray: scattered,
                color_atten: self.color,
            }),
            false => None,
        }
    }
}

fn reflect(v: Vec3, normal: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(normal) * *normal
}
