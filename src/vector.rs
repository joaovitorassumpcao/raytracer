struct Vec3 {
	x: f64,
	y: f64,
	z: f64,
}

impl Vec3 {
	
	pub fn dot(&self, ovec: &Vec3) -> f64 {
		self.x * ovec.x + self.y * ovec.y + self.z * ovec.z
	}

	pub fn len(&self) -> f64 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
	}
}