//      Copyright João Vitor Cunha Assumpção 2023.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use image::{ImageBuffer, RgbImage};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use material::Lambertian;
use object::{Scene, Sphere};
use rand::random;
use rayon::prelude::*;

mod camera;
mod material;
mod object;
mod ray;
mod vector;

fn main() {
    // Set up the image parameters
    const MAX_DEPTH: u8 = 50;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: u32 = 1920;
    const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;

    let camera = camera::Camera::default();

    let mut img: RgbImage = ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);

    let scene: Scene = vec![
        Box::new(Sphere::new(
            vec3!(0, 0, -1),
            0.5,
            Lambertian::new(vec3!(0.5)),
        )),
        Box::new(Sphere::new(
            vec3!(0, -100.5, -1),
            100.0,
            Lambertian::new(vec3!(0.5)),
        )),
    ];

    let bar: ProgressBar = ProgressBar::new((IMG_WIDTH * IMG_HEIGHT) as u64);

    bar.set_style(ProgressStyle::with_template(
        "{prefix:.bold.dim} {spinner:.green} {bar:50.white} {percent:}%  {elapsed_precise:.cyan} {msg:.green}",)
		.unwrap().progress_chars("▓░").tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "));

    img.enumerate_pixels_mut()
        .par_bridge()
        .progress_with(bar.clone())
        .for_each(|(i, j, pixel)| {
            let samples: u32 = 100;
            let mut colorpx = vec3!(0);

            // multisampling
            (0..=samples).for_each(|_| {
                // 0.0 <= t <= 1.0
                let u = (i as f64 + random::<f64>()) / (IMG_WIDTH - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (IMG_HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);
                colorpx = colorpx + ray::color(&ray, &scene, MAX_DEPTH);
            });

            // Calculate the color for the pixel using the ray
            *pixel = (colorpx / samples as f64).into();
        });

    bar.finish_with_message(" -> Done!");

    img.save("render.png").expect(" -> Render error!");
}
