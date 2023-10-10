use image::{GenericImageView, Rgba};
use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter File to load");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: &str = input.trim();
    let read_image_result = read_image(input);
    let rgba_array = match read_image_result {
        Ok(image) => image,
        Err(error) => panic!("Something went terribly wrong: {:?}", error),
    };
    println!("File Array lil sum like {:?}", rgba_array);
}

fn read_image(filename: &str) -> Result<Vec<u8>, image::ImageError> {
    let img = image::open(filename)?;
    let (width, height) = img.dimensions();
    let mut raw_pixels = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height{
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            raw_pixels.extend_from_slice(&pixel.0);
        }
    }
    Ok(raw_pixels)
}

fn calculate_grayscale_intensity(pixel: &Rgba<u8>) -> u8 {
        (0.3 * pixel[0] as f64 + 0.59 * pixel[1] as f64 + 0.11 * pixel[2] as f64) as u8
}

fn calculate_color_intensity(pixel: &Rgba<u8>) -> u8 {
    1
}
