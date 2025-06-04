mod color;
mod vec3;

use image::{Rgb, RgbImage};
// use std::error::Error;
use std::io::{self, Write};
use vec3::Vec3;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    let mut img = RgbImage::new(image_width, image_height);

    // Render
    println!("P3\n {} {} \n256\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", image_height - j);
        io::stdout().flush().unwrap();

        for i in 0..image_width {
            let pixel_color = Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            let rgb = color::write_color(pixel_color);
            img.put_pixel(i, j, Rgb([rgb.0 as u8, rgb.1 as u8, rgb.2 as u8]));
        }
    }

    eprintln!("\rDone.             \n");
    let path = "output.png";
    let _ = img.save(path);
    let _ = open::that(path);
}
