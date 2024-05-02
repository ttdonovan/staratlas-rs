use macroquad::prelude::*;

pub fn print_input(text: &str) {
    let center = get_text_center(text, Option::None, 30, 1.0, 0.0);

    draw_text_ex(
        text,
        screen_width() / 2.0 - center.x,
        screen_height() - 30.0,
        TextParams {
            font_size: 30,
            ..Default::default()
        },
    );
}

pub fn print_lines(lines: Vec<&str>, y_offset: f32) {
    let mut y = y_offset;

    for line in lines {
        draw_text_ex(line, 20.0, 20.0 + y, TextParams::default());
        y += 20.0;
    }
}
