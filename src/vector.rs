struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Color = Vec3;

pub type Point = Vec3;

impl Vec3 {
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn cross(&self, other: &Vec3) -> Self {
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
        let len = self.len();
        self.map(|x| x / len)
    }
}

impl From<Vec3> for image::Rgb<u8> {
    fn from(v: Vec3) -> Self {
        image::Rgb(
            [v.x, v.y, v.z].map(|c| (c * 255.999) as u8),
        )
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