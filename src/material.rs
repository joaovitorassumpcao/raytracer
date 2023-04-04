//      Copyright João Vitor Cunha Assumpção 2023.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use derive_more::Constructor;

use crate::{
    object::Hit,
    ray::Ray,
    vector::{rand_unitvec, Color, Vec3},
};

#[derive(Debug, Clone, Constructor)]
pub struct Reflection {
    pub ray: Ray,
    pub color_atten: Color,
}

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Metal {
    color: Color,
    fuzz: f64,
}

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Lambertian{
    color: Color
}

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Dielectric {
    ratio: f64
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

impl Material for Lambertian {
    fn scatter(&self, _incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let mut direction: Vec3 = hit.normal + rand_unitvec();

        if direction.is_zero() {
            direction = hit.normal;
        }

        let ray = Ray::new(hit.intersec, direction);

        Some(Reflection {
            ray,
            color_atten: self.color,
        })
    }
}

impl Material for Metal {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let reflection = reflect(incident_ray.direction, &hit.normal) + self.fuzz * rand_unitvec();
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

fn refract(incident_ray: &Ray, normal: &Vec3, ratio: f64) -> Vec3 {
    let dir = incident_ray.direction.normalize();
    let cos = -dir.dot(normal);
    let r_para = ratio * (dir + *normal * cos);
    let r_perp = -(1.0 - r_para.dot(&r_para)).abs().sqrt() * *normal;

    r_para + r_perp
}