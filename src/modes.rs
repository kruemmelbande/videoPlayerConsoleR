use image::Rgba;
use rand::{rngs::ThreadRng, Rng};

pub fn get_pixel_bw(pixel: &Rgba<u8>) -> u8 {
    return ((pixel[0] as i16 + pixel[2] as i16 + pixel[1] as i16) as i16 / 3 as i16) as u8;
}

pub fn get_dither(pixel: &Rgba<u8>, dither_ammount: u8, rng: &mut ThreadRng) -> u8 {
    let mut pixel_bw = get_pixel_bw(pixel);

    if pixel_bw < 255 - dither_ammount && pixel_bw > dither_ammount {
        pixel_bw -= dither_ammount;
        pixel_bw += rng.gen_range(0..=dither_ammount * 2);
    }

    return pixel_bw;
}

pub fn true_color(pixel: &Rgba<u8>, old_pixel: &Rgba<u8>) -> String {
    let img_string: String;

    if old_pixel == pixel {
        img_string = " ".to_string();
    } else {
        img_string = format!("\x1B[48;2;{};{};{}m ", pixel[0], pixel[1], pixel[2]);
    }

    return img_string;
}

pub fn ascii(pixel_bw: u8) -> String {
    return match pixel_bw {
        0..=15 => String::from(" "),
        16..=42 => String::from("."),
        43..=84 => String::from(","),
        85..=126 => String::from("-"),
        127..=168 => String::from("="),
        169..=210 => String::from("+"),
        211..=252 => String::from("*"),
        253..=255 => String::from("#"),
    };
}

pub fn block(pixel_bw: u8) -> String {
    return match pixel_bw {
        0..=42 => String::from(" "),
        43..=85 => String::from("░"),
        86..=128 => String::from("▒"),
        129..=170 => String::from("▓"),
        171..=255 => String::from("█"),
    };
}
