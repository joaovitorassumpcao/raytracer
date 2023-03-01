use image::{ImageBuffer, RgbImage};
use object::{Scene, Sphere};
use ray::Ray;
use rayon::prelude::*;
use vector::Point;

mod object;
mod ray;
mod vector;

fn main() {
    // Set up the image parameters
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 1920;
    let img_height = (img_width as f64 / aspect_ratio) as u32;

    // Set up the view parameters
    let view_height = 2.0;
    let view_width = aspect_ratio * view_height;
    let focal_len = 1.0;

    // Calculate the position of the top left corner of the view
    let origin: Point = vec3!(0);
    let horizontal_vec = vec3!(view_width, 0, 0);
    let vertical_vec = vec3!(0, -view_height, 0);
    let top_left: Point =
        origin - horizontal_vec / 2.0 - vertical_vec / 2.0 - vec3!(0, 0, focal_len);

    let mut img: RgbImage = ImageBuffer::new(img_width, img_height);

    let scene: Scene = vec![
        Box::new(Sphere::new(vec3!(0, 0, -1), 0.5)),
        Box::new(Sphere::new(vec3!(0, -100.5, -1), 100.0)),
    ];

    img.enumerate_pixels_mut().par_bridge().for_each(|(i, j, pixel)| {
        // 0.0 <= t <= 1.0
        let u = i as f64 / (img_width - 1) as f64;
        let v = j as f64 / (img_height - 1) as f64;

        // Calculate the ray direction
        let ray_dir = top_left + u * horizontal_vec + v * vertical_vec - origin;

        // Calculate the color for the pixel using the ray
        let ray = Ray::new(origin, ray_dir);
        *pixel = ray::color(&ray, &scene).into();
    });

    img.save("render.png").expect("image error");
}
