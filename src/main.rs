use image::GenericImageView;
use rodio::OutputStreamHandle;
use rodio::{source::Source, Decoder, OutputStream};
use term_size;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::{Duration, Instant};
use std::io::Write;
use console::Term;

fn clear_console() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}
fn calculate_divider(terminal_width: u32, terminal_height: u32, image_width: u32, image_height: u32) -> f32 {
    let aspect_ratio = (image_width as f32) / (image_height as f32);
    let terminal_aspect_ratio = (terminal_width as f32) / (terminal_height as f32);
    if aspect_ratio > terminal_aspect_ratio {
        (image_width as f32) / (terminal_width as f32)
    } else {
        (image_height as f32) / (terminal_height as f32)
    }
}

fn main() {
    let fps: f32 = 24.;
    //
    let folder_path = "video/";
    let f: usize = fs::read_dir(folder_path)
        .expect("Failed to read folder.")
        .count();
    let name = "apple-";
    let format = "png";
    let color: u32 = 0;
    //for full color, use 0 (looks best)
    //for ascii, use 1 (runs best on windows terminal)
    //for grayscale blocks, use 2 (runs terrible on windows terminal, might run as well as 1 on conhost or other terminals)
    //let divider = 9;
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
    let mut last_divider: f32 = 0.;
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
        print!("\x1B[H");
        let aspectratiocorrection: f32 = 2.; //because of non square characters, we assume that the image is twice as tall as it is wide
        
        //get terminal size
        let terminal_size = term_size::dimensions().unwrap();
        let terminal_width = terminal_size.0 as u32;
        let terminal_height = terminal_size.1 as u32;
        let float_divider = calculate_divider(terminal_width-1, terminal_height-1, width*2, height);
        if float_divider != last_divider {
            clear_console();
            last_divider = float_divider;
        }
        let new_height = (height as f32 / float_divider).floor() as u32;
        let new_width = ((width as f32 / float_divider).floor() * aspectratiocorrection) as u32 ;
        let mut pos_x: u32;
        let mut pos_y: u32;
        // Loop through each pixel in the image
        for y in 0..new_height{
            for x in 0..new_width{
                pos_x = (x as f32 * float_divider / aspectratiocorrection ) as u32;
                pos_y = (y as f32 * float_divider) as u32;
                let pixel = img.get_pixel(pos_x, pos_y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                if color == 0  {
                    let img_string = format!("\x1B[48;2;{r};{g};{b}m ", r = r, g = g, b = b);
                    print!("{}", img_string);
                } else if color == 1 {
                    let pixel_bw: u8 = ((r as i16 + b as i16 + g as i16) as i16 / 3 as i16) as u8;
                    match pixel_bw {
                        0..=15 => print!(" "),
                        16..=42 => print!("."),
                        43..=84 => print!(","),
                        85..=126 => print!("-"),
                        127..=168 => print!("="),
                        169..=210 => print!("+"),
                        211..=252 => print!("*"),
                        253..=255 => print!("#")
                    }
                } else {
                    let pixel_bw = ((r as i16 + b as i16 + g as i16) as i16 / 3 as i16) as u8;
                    match pixel_bw {
                        0..=42 => print!(" "),
                        43..=85 => print!("░"),
                        86..=128 => print!("▒"),
                        129..=170 => print!("▓"),
                        171..=255 => print!("█")

                    }
                }
            }
            println!("");
        }

        std::io::stdout().flush().unwrap();
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
