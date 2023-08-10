use std::time::{Duration, Instant};
use image::GenericImageView;


fn main() {
    let n = 33; // loop every 1000 milliseconds
    let mut last_time = Instant::now();
    for frame in 1..6571{

        // Open the image file
        let path: String=format!("video/apple-{:0width$}.png",frame,width=5);
        //println!("{}", path );
        let img = image::open(path).unwrap();
        print!("\x1B[1;1H");
        // Get the dimensions of the image
        let (width, height) = img.dimensions();
        let divider=6;
        // Loop through each pixel in the image
        for y in (0..height).step_by(divider) {
            for x in (0..width).step_by(divider/2) {

                // Get the color of the pixel at (x, y)
                let pixel = img.get_pixel(x, y);

                // Get the RGB values of the pixel
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

                // Do something with the RGB values
                //println!("Pixel at ({}, {}) has RGB values ({}, {}, {})", x, y, r, g, b);
                let pixel_bw:u8=((r as i16+b as i16+g as i16) as i16/3 as i16) as u8;
                if pixel_bw<127{
                    print!(" ");
                }else{
                    print!("#");
                }
            }
            println!();
        }
        // Calculate the time it took to execute the code inside the loop
        let elapsed = last_time.elapsed();
        let elapsed_millis = elapsed.as_millis() as u64;

        // Sleep for the remaining time until the next loop iteration
        if elapsed_millis < n {
            let remaining = Duration::from_millis(n - elapsed_millis);
            std::thread::sleep(remaining);
        }

        // Update the last_time variable
        last_time = Instant::now();
    }
}