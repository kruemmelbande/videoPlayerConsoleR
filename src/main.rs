use image::{GenericImageView, Rgba};
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle};
use std::{
    env, fs,
    fs::File,
    io::{BufReader, Write},
    process, thread,
    time::{Duration, Instant},
};
use term_size;

mod modes;

use video_player_console_r::{calculate_divider, clear_console, VideoOptions};

fn main() {
    let args: Vec<String> = env::args().collect();
    let options: VideoOptions;

    if args.len() == 5 {
        options = VideoOptions {
            fps: args[1].parse().expect("That isn't a valid fps"),
            color_mode: args[2].parse().expect("That isn't a valid color mode"),
            audio: args[3].parse().expect("That isn't a valid audio mode"),
            mode_option: args[4].parse().expect("That isn't a valid mode option"),
        };
    } else if args.len() == 1 {
        options = VideoOptions {
            fps: 25.,
            color_mode: 0,
            audio: true,
            mode_option: 10,
        };
    } else {
        eprintln!("Options must be: <fps count> <color mode> <audio toggle> <mode option>");
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

    clear_console();

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

                let mut pixel = img.get_pixel(pos_x, pos_y);

                let pixel_string: String;

                pixel_string = match options.color_mode {
                    0 => modes::true_color(&pixel, &old_pixel),
                    1 => {
                        let col_div = options.mode_option;

                        pixel = Rgba([
                            (pixel[0] + col_div / 2) / col_div * col_div,
                            (pixel[1] + col_div / 2) / col_div * col_div,
                            (pixel[2] + col_div / 2) / col_div * col_div,
                            0,
                        ]);

                        modes::true_color(&pixel, &old_pixel)
                    }
                    2 => modes::ascii(modes::get_pixel_bw(&pixel)),
                    3 => modes::ascii(modes::get_dither(&pixel, options.mode_option, &mut rng)),
                    4 => modes::block(modes::get_pixel_bw(&pixel)),
                    5 => modes::block(modes::get_dither(&pixel, options.mode_option, &mut rng)),
                    _ => modes::true_color(&pixel, &old_pixel),
                };

                write!(lock, "{}", pixel_string).expect("error writing to stdout");

                old_pixel = pixel;
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

    println!(
        "\nSkipped {} out of {} frames. ({}%)",
        frames_skip,
        f,
        frames_skip as f32 / f as f32 * 100.
    );
}
