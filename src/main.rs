use image::{RgbImage, ImageBuffer, Rgb, ImageError};
use uuid::Uuid;

fn save_image(buffer:RgbImage)->Result<String,ImageError>{
    let uuid = Uuid::now_v7();
    let mut file_name = uuid.to_string();
    file_name.push_str(".png");
    buffer.save(&file_name).map(|_|file_name)
}


fn main() {

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let r = x as f64 / (IMAGE_WIDTH-1) as f64;
        let g = y as f64 / (IMAGE_HEIGHT-1) as f64;
        let b = 0.25;

        let ir = (u8::max_value() as f64 * r) as u8;
        let ig = (u8::max_value() as f64 * g) as u8;
        let ib = (u8::max_value() as f64 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    match save_image(buffer){
        Ok(file_name)=>println!("Printed to: {file_name}"),
        Err(_)=>eprintln!("Error")
    }
}