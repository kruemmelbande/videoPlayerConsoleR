use image::GenericImageView;
use rodio::OutputStreamHandle;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let fps: f32 = 24.;
    //
    let folder_path = "video/";
    let f: usize = fs::read_dir(folder_path)
        .expect("Failed to read folder.")
        .count();
    let name = "apple-";
    let format = "png";
    let color: bool = true;
    let divider = 11;
    let enable_audio = true;

    let _stream: OutputStream;
    let stream_handle: OutputStreamHandle;
    let file: BufReader<File>;
    let source: Decoder<BufReader<File>>;
    if enable_audio {
            (_stream, stream_handle) = OutputStream::try_default().unwrap();
            file = BufReader::new(File::open("audio.mp3").unwrap());
            source = Decoder::new(file).unwrap();
            stream_handle.play_raw(source.convert_samples()).ok();
    }

    let start = Instant::now();
    let n: u64 = (1000000. / fps as f32) as u64; // loop every n micros
    let mut frames_skip: u64 = 0;
    for frame in 1..f {
        //if we are supposed to be in the next frame, just skip this one
        if start.elapsed().as_micros() > (((frame as u128) + 1) * n as u128) {
            frames_skip += 1;
            continue;
        }
        // Open the image file
        let path: String = format!("{folder_path}/{name}{:0width$}.{format}", frame, width = 5);
        //println!("{}", path );
        let img = image::open(path).expect("The image has not been found in the specified path, or under the specified name.");
        // Get the dimensions of the image
        let (width, height) = img.dimensions();
        print!("\x1B[1;1H");
        // Loop through each pixel in the image
        for y in (0..height).step_by(divider) {
            for x in (0..width).step_by(divider / 2) {
                // Get the color of the pixel at (x, y)
                let pixel = img.get_pixel(x, y);

                // Get the RGB values of the pixel
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

                // Do something with the RGB values
                //println!("Pixel at ({}, {}) has RGB values ({}, {}, {})", x, y, r, g, b);
                if color {
                    print!("\x1B[48;2;{};{};{}m ", r, g, b);
                } else {
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
            }
            println!();
        }
        //println!("{}",frame);
        // Calculate the time it took to execute the code inside the loop
        // Calculate the target execution time for this iteration
        let target_time = start + Duration::from_micros(frame as u64 * n as u64);

        // Get the current time
        let current_time = Instant::now();

        // Check if we need to sleep or if we're already behind schedule
        if target_time > current_time {
            // Calculate the duration to sleep to reach the target time
            let sleep_duration = target_time - current_time;

            // Sleep until the target time
            thread::sleep(sleep_duration);
        }
    }
    print!("\x1B[0m");
    println!("Skipped {} out of {} frames. ({}%)", frames_skip, f, frames_skip as f32 / f as f32 * 100.);
}
