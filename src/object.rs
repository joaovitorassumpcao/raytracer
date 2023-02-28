use derive_more::Constructor;

pub trait Object {
    fn hit(&self, ray: &Ray, bounds: (f64, f64)) -> Option<Hit>;
}

pub struct Hit {
    intersec: Point,
    normal: Vec3,
    t: f64,
}

#[derive(Debug, Constructor)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let v = ray.direction;
        let oc = ray.origin - self.center;
        let a = v.dot(&v);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}
