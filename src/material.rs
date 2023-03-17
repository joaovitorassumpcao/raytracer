use crate::{object::Hit, ray::Ray, vector::Color};

pub struct Reflection {
    ray: Ray,
	atten_color: Color,
}

pub trait Material {
    fn scatter(&self, incident_ray: &Ray, hit: &Hit) -> Option<Reflection>;
}

pub struct Lambertian(Color);