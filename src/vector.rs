use derive_more::{Add, Constructor, Div, Mul, Neg, Sub};

#[derive(Constructor, Debug, PartialEq, Clone, Copy, PartialOrd, Add, Sub, Mul, Div, Neg)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;

pub type Point = Vec3;

pub type Direction = Vec3;

#[macro_export]
macro_rules! vec3 {
    ($x: expr) => {
        $crate::vector::Vec3::new(f64::from($x), f64::from($x), f64::from($x))
    };

    ($x: expr, $y: expr, $z: expr) => {
        $crate::vector::Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };
}

impl Vec3 {
    pub fn dot(&self, other: &Self) -> f64 {
        // x1 * x2 + y1 * y2 + z1 * z2
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> f64 {
        // sqrt(x^2 + y^2 + z^2)
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(f64) -> f64,
    {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }

    /// Linearly interpolate between two vectors
    pub fn lerp(start: Self, end: Self, t: f64) -> Self {
        start * t + end * (1.0 - t)
    }
}

impl From<Vec3> for image::Rgb<u8> {
    fn from(v: Vec3) -> Self {
        image::Rgb([v.x, v.y, v.z].map(|c| (c * 255.999) as u8))
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| x * self)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
