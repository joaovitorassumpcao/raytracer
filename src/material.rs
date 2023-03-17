use derive_more::Constructor;

use crate::{
    object::Hit,
    ray::Ray,
    vector::{Color, Vec3},
};

#[derive(Debug, Clone, Constructor)]
pub struct Reflection {
    ray: Ray,
    atten_color: Color,
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

pub struct Lambertian(Color);

impl Material for Lambertian {
    fn scatter(&self, _incident_ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let direction = hit.normal + Vec3::rand_unitvec();

        //let origin = hit.intersec;
        //0.5 * color(&Ray::new(origin, direction), scene, depth - 1)

        let ray = Ray::new(hit.intersec, direction);
        Some(Reflection {
            ray,
            atten_color: self.0,
        })
    }
}
