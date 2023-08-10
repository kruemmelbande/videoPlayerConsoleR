use std::thread;
use std::time::Duration;
use image::GenericImageView;


fn main() {
    for n in 1..6571{
    // Open the image file
    let path: String=format!("video/apple-{:0width$}.png",n,width=5);
    println!("{}", path );
    let img = image::open(path).unwrap();
    print!("\x1B[1;1H");
    // Get the dimensions of the image
    let (width, height) = img.dimensions();
    let divider=8;
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
    thread::sleep(Duration::from_millis(33))
    
}
}