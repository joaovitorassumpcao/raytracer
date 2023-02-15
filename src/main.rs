use image::{ImageBuffer, RgbImage};

fn main() {
    let mut img: RgbImage = ImageBuffer::new(256, 256);

    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([255,0,0]);
    }

    img.save("red.png").expect("image error");
}
