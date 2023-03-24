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
