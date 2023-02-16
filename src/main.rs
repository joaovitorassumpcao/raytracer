use image::{ImageBuffer, RgbImage};

mod vector;

fn main() {
    let (w, h) = (300, 300);
    let mut img: RgbImage = ImageBuffer::new(256, 256);

    for (i, j, pixel) in img.enumerate_pixels_mut() {
        let r = i as f64 / (w - 1) as f64;
        let g = j as f64 / (h - 1) as f64;
        let b = 0.25;
        *pixel = image::Rgb([r,g,b].map(|x| (x * 255.999) as u8));
    }

    img.save("gradient.png").expect("image error");
}
