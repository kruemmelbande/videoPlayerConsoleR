use console::Term;
use image::{GenericImageView, Rgba};
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

    let f: usize = fs::read_dir("video/")
        .expect("Failed to read folder.")
        .count();
    let mut rng = rand::thread_rng();
    let stdout = std::io::stdout();

    if options.audio {
        let _stream: OutputStream;
        let stream_handle: OutputStreamHandle;

        (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open("audio.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples()).ok();
    }

    const ASPECT_RATIO_CORRECTION: f32 = 2.; //because of non square characters, we assume that the image is twice as tall as it is wide
    let start_time = Instant::now();
    let n: u64 = (1000000. / options.fps as f32) as u64; // loop every n micros
    let mut frames_skip: u64 = 0;
    let mut last_divider: f32 = 0.;

    println!("\x1Bc");

    for frame in 1..f {
        let mut lock = stdout.lock();

        if start_time.elapsed().as_micros() > (((frame as u128) + 1) * n as u128) {
            frames_skip += 1;
            continue;
        }

        let img = image::open(format!("video/apple-{:0width$}.png", frame, width = 5)).expect(
            "The image has not been found in the specified path, or under the specified name.",
        );

        let (width, height) = img.dimensions();

        let terminal_size = term_size::dimensions().unwrap();
        let float_divider = calculate_divider(
            terminal_size.0 as u32 - 1,
            terminal_size.1 as u32 - 1,
            (width as f32 * ASPECT_RATIO_CORRECTION).floor() as u32,
            height,
        );

        if float_divider != last_divider {
            clear_console();
            last_divider = float_divider;
        }

        let mut old_pixel: Rgba<u8> = Rgba([0, 0, 0, 0]);

        print!("\x1B[H");
        for y in 0..(height as f32 / float_divider).floor() as u32 {
            for x in 0..((width as f32 / float_divider).floor() * ASPECT_RATIO_CORRECTION) as u32 {
                let pos_x = (x as f32 * float_divider / ASPECT_RATIO_CORRECTION) as u32;
                let pos_y = (y as f32 * float_divider) as u32;

                let pixel = img.get_pixel(pos_x, pos_y);

                if options.color_mode == 0 {
                    let img_string: String;

                    if old_pixel == pixel {
                        img_string = " ".to_string();
                    } else {
                        img_string = format!("\x1B[48;2;{};{};{}m ", pixel[0], pixel[1], pixel[2]);
                    }

                    old_pixel = pixel;

                    write!(lock, "{}", img_string).expect("error writing to stdout");
                } else if options.color_mode == 1 {
                    let pixel_bw: u8 = ((pixel[0] as i16 + pixel[2] as i16 + pixel[1] as i16)
                        as i16
                        / 3 as i16) as u8;
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
                    let pixel_bw = ((pixel[0] as i16 + pixel[2] as i16 + pixel[1] as i16) as i16
                        / 3 as i16) as u8;
                    match pixel_bw {
                        0..=42 => write!(lock, " ").expect("error writing to stdout"),
                        43..=85 => write!(lock, "░").expect("error writing to stdout"),
                        86..=128 => write!(lock, "▒").expect("error writing to stdout"),
                        129..=170 => write!(lock, "▓").expect("error writing to stdout"),
                        171..=255 => write!(lock, "█").expect("error writing to stdout"),
                    }
                } else if options.color_mode == 3 {
                    let mut pixel_bw = ((pixel[0] as i16 + pixel[2] as i16 + pixel[1] as i16)
                        as i16
                        / 3 as i16) as u8;
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

                    let fpixel: Rgba<u8> = Rgba([
                        (pixel[0] + col_div / 2) / col_div * col_div,
                        (pixel[1] + col_div / 2) / col_div * col_div,
                        (pixel[2] + col_div / 2) / col_div * col_div,
                        0,
                    ]);

                    if old_pixel == fpixel {
                        img_string = " ".to_string();
                    } else {
                        img_string =
                            format!("\x1B[48;2;{};{};{}m ", fpixel[0], fpixel[1], fpixel[2]);
                    }

                    old_pixel = fpixel;

                    write!(lock, "{}", img_string).expect("error writing to stdout");
                } else {
                    let mut pixel_bw: u8 = ((pixel[0] as i16 + pixel[2] as i16 + pixel[1] as i16)
                        as i16
                        / 3 as i16) as u8;
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
        let target_time = start_time + Duration::from_micros(frame as u64 * n as u64);

        let current_time = Instant::now();

        if target_time > current_time {
            let sleep_duration = target_time - current_time;

            thread::sleep(sleep_duration);
        }
    }

    // println!("\x1B[0m");
    println!(
        "\nSkipped {} out of {} frames. ({}%)",
        frames_skip,
        f,
        frames_skip as f32 / f as f32 * 100.
    );
}
