use std::{default, process};

use console::Term;

pub fn clear_console() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

pub fn calculate_divider(
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

pub fn throw_err(msg: &str) {
    eprintln!("{}", msg);
    process::exit(1);
}

pub struct VideoOptions {
    pub fps: f32,
    pub color_mode: u8,
    pub audio: bool,
    pub mode_option: u8,
}

pub fn modify_option(flag: &str, args: &Vec<String>, default: String) -> String {
    if let Some(i) = args.iter().position(|x| x == flag) {
        if args.len() <= i + 1 {
            throw_err("Please specify a fps value after --fps");
        }

        return args[i + 1].clone();
    }

    return default;
}

pub fn get_options(args: &Vec<String>) -> VideoOptions {
    let mut options = VideoOptions {
        fps: 60.0,
        color_mode: 1,
        audio: false,
        mode_option: 10,
    };

    options.fps = match modify_option("--fps", args, options.fps.to_string()).parse() {
        Ok(fps) => {
            if fps > 0.0 {
                fps
            } else {
                throw_err("Please specify a valid fps value after --fps");
                default::Default::default()
            }
        }
        Err(_) => {
            throw_err("Please specify a valid fps value after --fps");
            default::Default::default()
        }
    };

    options.color_mode = match modify_option("--mode", args, options.color_mode.to_string()).parse()
    {
        Ok(color_mode) => {
            if color_mode < 6 {
                color_mode
            } else {
                throw_err("Please specify a valid mode after --mode");
                default::Default::default()
            }
        }
        Err(_) => {
            throw_err("Please specify a valid fps value after --mode");
            default::Default::default()
        }
    };

    options.audio = match modify_option("--audio", args, options.audio.to_string()).parse() {
        Ok(audio) => audio,
        Err(_) => {
            throw_err("Please specify a valid boolean after --audio");
            default::Default::default()
        }
    };

    options.mode_option =
        match modify_option("--option", args, options.mode_option.to_string()).parse() {
            Ok(mode_option) => mode_option,
            Err(_) => {
                throw_err("Please specify a valid fps value after --fps");
                default::Default::default()
            }
        };

    return options;
}
