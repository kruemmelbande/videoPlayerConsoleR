use image::GenericImageView;
use std::fs;
use std::time::{Duration, Instant};

fn main() {
    let fps:f32 = 25.0;
    //
    let folder_path = "video/";
    let f: usize = fs::read_dir(folder_path)
        .expect("Failed to read folder.")
        .count();
    let name="apple-";
    let format ="png";
    let divider = 6;
    let mut last_time = Instant::now();
    let n = 1000 as f32/fps as f32; // loop every n millis
    
    let micros= (n*1000 as f32) as u64;
    
    for frame in 1..f {
        // Open the image file
        let path: String = format!("{folder_path}/{name}{:0width$}.{format}", frame, width = 5);
        //println!("{}", path );
        let img = image::open(path).unwrap();
        print!("\x1B[1;1H");
        // Get the dimensions of the image
        let (width, height) = img.dimensions();

        // Loop through each pixel in the image
        for y in (0..height).step_by(divider) {
            for x in (0..width).step_by(divider / 2) {
                // Get the color of the pixel at (x, y)
                let pixel = img.get_pixel(x, y);

                // Get the RGB values of the pixel
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

                // Do something with the RGB values
                //println!("Pixel at ({}, {}) has RGB values ({}, {}, {})", x, y, r, g, b);
                let pixel_bw: u8 = ((r as i16 + b as i16 + g as i16) as i16 / 3 as i16) as u8;
                match pixel_bw {
                    0..=42 => print!(" "),
                    43..=84 => print!("."),
                    85..=126 => print!("-"),
                    127..=168 => print!("="),
                    169..=210 => print!("+"),
                    211..=252 => print!("*"),
                    253..=255 => print!("#"),
                }
            }
            println!();
        }
        //println!("{}",frame);
        // Calculate the time it took to execute the code inside the loop
        let elapsed = last_time.elapsed();
        let elapsed_micros = elapsed.as_micros() as u64;

        // Sleep for the remaining time until the next loop iteration
        if elapsed_micros < micros {
            let remaining = Duration::from_micros(micros - elapsed_micros);
            std::thread::sleep(remaining);
        }

        // Update the last_time variable
        last_time = Instant::now();
    }
}
