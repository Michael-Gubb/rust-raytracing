mod vector;
mod ray;

use image::{ImageBuffer, ImageError, Rgb, RgbImage};
use std::u8;
use uuid::Uuid;

fn save_image(buffer: RgbImage) -> Result<String, ImageError> {
    const IMAGE_FOLDER: &'static str = "./images/";
    let uuid = Uuid::now_v7();
    let mut file_name = String::from(IMAGE_FOLDER);
    file_name.push_str(&uuid.to_string());
    file_name.push_str(".png");
    buffer.save(&file_name).map(|_| file_name)
}

fn main() {
    const IMAGE_WIDTH: u32 = u8::MAX as u32 + 1;
    const IMAGE_HEIGHT: u32 = u8::MAX as u32 + 1;

    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let r = x as f64 / (u8::MAX) as f64;
        let g = y as f64 / (u8::MAX) as f64;
        let b = 0.25;

        let ir = (u8::MAX as f64 * r) as u8;
        let ig = (u8::MAX as f64 * g) as u8;
        let ib = (u8::MAX as f64 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    match save_image(buffer) {
        Ok(file_name) => {
            println!("Printed to: {file_name}")
        }
        Err(err) => {
            eprintln!("Error while saving image: {:?}", err)
        }
    }
}
