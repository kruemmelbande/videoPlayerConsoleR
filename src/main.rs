use console::Term;
use image::GenericImageView;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle};
use std::{
    env, fs,
    fs::File,
    io::{BufReader, Write},
    process, thread,
    time::{Duration, Instant},
};
use term_size;

fn clear_console() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}
fn calculate_divider(
    terminal_width: u32,
    terminal_height: u32,
    image_width: u32,
    image_height: u32,
) -> f32 {
    let aspect_ratio = (image_width as f32) / (image_height as f32);
    let terminal_aspect_ratio = (terminal_width as f32) / (terminal_height as f32);
    if aspect_ratio > terminal_aspect_ratio {
        (image_width as f32) / (terminal_width as f32)
    } else {
        (image_height as f32) / (terminal_height as f32)
    }
}

struct VideoOptions {
    fps: f32,
    color_mode: u8,
    audio: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let options: VideoOptions;

    if args.len() == 4 {
        options = VideoOptions {
            fps: args[1].parse().expect("That isnt a valid fps"),
            color_mode: args[2].parse().expect("That isnt a valid color mode"),
            audio: args[3].parse().expect("That isnt a valid audio mode"),
        };
    } else if args.len() == 1 {
        options = VideoOptions {
            fps: 25.,
            color_mode: 0,
            audio: true,
        };
    } else {
        eprintln!("Options must be: <fps count> <color mode> <audio toggle>");
        process::exit(1);
    }

    let folder_path = "video/";
    let f: usize = fs::read_dir(folder_path)
        .expect("Failed to read folder.")
        .count();
    let name = "apple-";
    let format = "png";
    let mut rng = rand::thread_rng();

    let stdout = std::io::stdout();
    let _stream: OutputStream;
    let stream_handle: OutputStreamHandle;
    let file: BufReader<File>;
    let source: Decoder<BufReader<File>>;
    if options.audio {
        (_stream, stream_handle) = OutputStream::try_default().unwrap();
        file = BufReader::new(File::open("audio.mp3").unwrap());
        source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples()).ok();
    }

    let start = Instant::now();
    let n: u64 = (1000000. / options.fps as f32) as u64; // loop every n micros

    println!("\x1Bc");
    let mut frames_skip: u64 = 0;
    let mut last_divider: f32 = 0.;
    for frame in 1..f {
        let mut lock = stdout.lock();
        if start.elapsed().as_micros() > (((frame as u128) + 1) * n as u128) {
            frames_skip += 1;
            continue;
        }
        let path: String = format!("{folder_path}/{name}{:0width$}.{format}", frame, width = 5);
        let img = image::open(path).expect(
            "The image has not been found in the specified path, or under the specified name.",
        );
        let (width, height) = img.dimensions();
        print!("\x1B[H");
        let aspectratiocorrection: f32 = 2.; //because of non square characters, we assume that the image is twice as tall as it is wide

        let terminal_size = term_size::dimensions().unwrap();
        let terminal_width = terminal_size.0 as u32;
        let terminal_height = terminal_size.1 as u32;
        let float_divider = calculate_divider(
            terminal_width - 1,
            terminal_height - 1,
            (width as f32 * aspectratiocorrection).floor() as u32,
            height,
        );
        if float_divider != last_divider {
            clear_console();
            last_divider = float_divider;
        }
        let new_height = (height as f32 / float_divider).floor() as u32;
        let new_width = ((width as f32 / float_divider).floor() * aspectratiocorrection) as u32;
        let mut pos_x: u32;
        let mut pos_y: u32;

        let mut old_r: u8 = 0;
        let mut old_g: u8 = 0;
        let mut old_b: u8 = 0;

        for y in 0..new_height {
            for x in 0..new_width {
                pos_x = (x as f32 * float_divider / aspectratiocorrection) as u32;
                pos_y = (y as f32 * float_divider) as u32;
                let pixel = img.get_pixel(pos_x, pos_y);
                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                if options.color_mode == 0 {
                    let img_string: String;
                    if old_r == r && old_b == b && old_g == g {
                        img_string = " ".to_string();
                    } else {
                        img_string = format!("\x1B[48;2;{r};{g};{b}m ", r = r, g = g, b = b);
                    }

                    (old_r, old_g, old_b) = (r, g, b);

                    write!(lock, "{}", img_string).expect("error writing to stdout");
                } else if options.color_mode == 1 {
                    let pixel_bw: u8 = ((r as i16 + b as i16 + g as i16) as i16 / 3 as i16) as u8;
                    match pixel_bw {
                        0..=15 => write!(lock, " ").expect("error writing to stdout"),
                        16..=42 => write!(lock, ".").expect("error writing to stdout"),
                        43..=84 => write!(lock, ",").expect("error writing to stdout"),
                        85..=126 => write!(lock, "-").expect("error writing to stdout"),
                        127..=168 => write!(lock, "=").expect("error writing to stdout"),
                        169..=210 => write!(lock, "+").expect("error writing to stdout"),
                        211..=252 => write!(lock, "*").expect("error writing to stdout"),
                        253..=255 => write!(lock, "#").expect("error writing to stdout"),
                    }
                } else if options.color_mode == 2 {
                    let pixel_bw = ((r as i16 + b as i16 + g as i16) as i16 / 3 as i16) as u8;
                    match pixel_bw {
                        0..=42 => write!(lock, " ").expect("error writing to stdout"),
                        43..=85 => write!(lock, "░").expect("error writing to stdout"),
                        86..=128 => write!(lock, "▒").expect("error writing to stdout"),
                        129..=170 => write!(lock, "▓").expect("error writing to stdout"),
                        171..=255 => write!(lock, "█").expect("error writing to stdout"),
                    }
                } else if options.color_mode == 3 {
                    let mut pixel_bw = ((r as i16 + b as i16 + g as i16) as i16 / 3 as i16) as u8;
                    let dither_ammount = 40;
                    if pixel_bw < 255 - dither_ammount && pixel_bw > dither_ammount {
                        pixel_bw -= dither_ammount;
                        pixel_bw += rng.gen_range(0..=dither_ammount * 2);
                    }
                    match pixel_bw {
                        0..=42 => write!(lock, " ").expect("error writing to stdout"),
                        43..=85 => write!(lock, "░").expect("error writing to stdout"),
                        86..=128 => write!(lock, "▒").expect("error writing to stdout"),
                        129..=170 => write!(lock, "▓").expect("error writing to stdout"),
                        171..=255 => write!(lock, "█").expect("error writing to stdout"),
                    }
                } else if options.color_mode == 4 {
                    let img_string: String;

                    let col_div: u8 = 10;

                    let fr = ((r / col_div) as f32).floor() as u8 * col_div;
                    let fg = ((g / col_div) as f32).floor() as u8 * col_div;
                    let fb = ((b / col_div) as f32).floor() as u8 * col_div;

                    if old_r == fr && old_b == fb && old_g == fg {
                        img_string = " ".to_string();
                    } else {
                        img_string = format!("\x1B[48;2;{r};{g};{b}m ", r = fr, g = fg, b = fb);
                    }

                    (old_r, old_g, old_b) = (fr, fg, fb);

                    write!(lock, "{}", img_string).expect("error writing to stdout");
                } else {
                    let mut pixel_bw: u8 =
                        ((r as i16 + b as i16 + g as i16) as i16 / 3 as i16) as u8;
                    let dither_ammount: u8 = 20;
                    if pixel_bw < 255 - dither_ammount && pixel_bw > dither_ammount {
                        pixel_bw -= dither_ammount;
                        pixel_bw += rng.gen_range(0..=dither_ammount * 2);
                    }
                    match pixel_bw {
                        0..=15 => write!(lock, " ").expect("error writing to stdout"),
                        16..=42 => write!(lock, ".").expect("error writing to stdout"),
                        43..=84 => write!(lock, ",").expect("error writing to stdout"),
                        85..=126 => write!(lock, "-").expect("error writing to stdout"),
                        127..=168 => write!(lock, "=").expect("error writing to stdout"),
                        169..=210 => write!(lock, "+").expect("error writing to stdout"),
                        211..=252 => write!(lock, "*").expect("error writing to stdout"),
                        253..=255 => write!(lock, "#").expect("error writing to stdout"),
                    }
                }
            }
            write!(lock, "\n").expect("error writing to stdout");
        }

        std::io::stdout().flush().unwrap();
        let target_time = start + Duration::from_micros(frame as u64 * n as u64);

        let current_time = Instant::now();

        if target_time > current_time {
            let sleep_duration = target_time - current_time;

            thread::sleep(sleep_duration);
        }
    }
    print!("\x1B[0m");
    println!(
        "Skipped {} out of {} frames. ({}%)",
        frames_skip,
        f,
        frames_skip as f32 / f as f32 * 100.
    );
}
