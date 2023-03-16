use image::{ImageBuffer, RgbImage};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use object::{Scene, Sphere};
use rand::random;
use rayon::prelude::*;

mod camera;
mod object;
mod ray;
mod vector;

fn main() {
    let max_depth: u8 = 50;

    let camera = camera::Camera::default();

    // Set up the image parameters
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 800;
    let img_height = (img_width as f64 / aspect_ratio) as u32;

    let mut img: RgbImage = ImageBuffer::new(img_width, img_height);

    let scene: Scene = vec![
        Box::new(Sphere::new(vec3!(0, 0, -1), 0.5)),
        Box::new(Sphere::new(vec3!(0, -100.5, -1), 100.0)),
    ];

    let bar: ProgressBar = ProgressBar::new((img_width * img_height) as u64);

    bar.set_style(ProgressStyle::with_template(
        "{prefix:.bold.dim} {spinner:.green} {bar:50.white} {percent:}%  {elapsed_precise:.cyan} {msg:.green}",)
		.unwrap().progress_chars("▓░").tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "));

    img.enumerate_pixels_mut()
        .par_bridge()
        .progress_with(bar.clone())
        .for_each(|(i, j, pixel)| {
            let samples: u32 = 100;
            let mut colorpx = vec3!(0);

            (0..=samples).for_each(|_| {
                // 0.0 <= t <= 1.0
                let u = (i as f64 + random::<f64>()) / (img_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (img_height - 1) as f64;

                // Calculate the ray direction
                let ray = camera.get_ray(u, v);
                colorpx = colorpx + ray::color(&ray, &scene, max_depth);
            });

            // Calculate the color for the pixel using the ray
            *pixel = (colorpx / samples as f64).into();
        });
    bar.finish_with_message(" -> Done!");

    img.save("render.png").expect("image error");
}
