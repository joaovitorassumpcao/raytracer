use image::{ImageBuffer, RgbImage};
use object::{Scene, Sphere};
use rayon::prelude::*;
use rand::random;

mod object;
mod ray;
mod vector;
mod camera;

fn main() {
	let camera = camera::Camera::default();

	// Set up the image parameters
	let aspect_ratio = 16.0 / 9.0;
	let img_width: u32 = 400;
	let img_height = (img_width as f64 / aspect_ratio) as u32;
	

	let mut img: RgbImage = ImageBuffer::new(img_width, img_height);

	let scene: Scene = vec![
		Box::new(Sphere::new(vec3!(0, 0, -1), 0.5)),
		Box::new(Sphere::new(vec3!(0, -100.5, -1), 100.0)),
	];

	img.enumerate_pixels_mut().par_bridge().for_each(|(i, j, pixel)| {
		let samples: u32 = 100;
		let mut colorpx = vec3!(0);
		for _ in 0..=samples {
			// 0.0 <= t <= 1.0
			let u = (i as f64 + random::<f64>()) / (img_width - 1) as f64;
			let v = (j as f64 + random::<f64>()) / (img_height - 1) as f64;

			// Calculate the ray direction
			let ray = camera.get_ray(u, v);
			colorpx = colorpx + ray::color(&ray, &scene);
		}

		// Calculate the color for the pixel using the ray
		*pixel = (colorpx / samples as f64).into();

	});

	img.save("render.png").expect("image error");
}
