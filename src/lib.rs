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

pub struct VideoOptions {
    pub fps: f32,
    pub color_mode: u8,
    pub audio: bool,
    pub mode_option: u8,
}
