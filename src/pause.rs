use macroquad::prelude::*;
pub fn draw_pause(volume: i32) -> i32 {
    draw_pause_message();
    draw_change_volume(volume)
}

fn draw_pause_message() {
    crate::canvas::draw_screen(Color::new(0.1, 0.1, 0.1, 0.8));
    let text = "Pause";
    let font_size = 120.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        screen_width() / 2. - str_size.width / 2.,
        120.,
        font_size,
        RED,
    );
    let text = "Press Escape key";
    let font_size = 60.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        screen_width() / 2. - str_size.width / 2.,
        210.,
        font_size,
        RED,
    );
    let text = "to resume game";
    let font_size = 60.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        screen_width() / 2. - str_size.width / 2.,
        270.,
        font_size,
        RED,
    );
}

fn draw_change_volume(mut volume: i32) -> i32 {
    let text = "Change volume(0~100)";
    let font_size = 60.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        screen_width() / 2. - str_size.width / 2.,
        360.,
        font_size,
        WHITE,
    );
    let mut up_color_thick = (WHITE, 1.);
    let mut down_color_thick = (WHITE, 1.);
    if is_key_down(KeyCode::Up) {
        volume += 1;
        up_color_thick = (YELLOW, 4.);
    }
    if is_key_down(KeyCode::Down) {
        volume -= 1;
        down_color_thick = (YELLOW, 4.);
    }
    if volume < 0 {
        volume = 0;
    }
    if 100 < volume {
        volume = 100;
    }
    let top = Vec2::new(screen_width() / 2., 420.);
    draw_triangle_lines(
        top,
        Vec2::new(top.x - 15., top.y + 40.),
        Vec2::new(top.x + 15., top.y + 40.),
        up_color_thick.1,
        up_color_thick.0,
    );
    let text = &volume.to_string();
    let str_size = measure_text(text, None, font_size as _, 1.0);
    draw_text(
        text,
        screen_width() / 2. - str_size.width / 2.,
        520.,
        font_size,
        WHITE,
    );
    let bottom = Vec2::new(screen_width() / 2., 590.);
    draw_triangle_lines(
        bottom,
        Vec2::new(bottom.x - 15., bottom.y - 40.),
        Vec2::new(bottom.x + 15., bottom.y - 40.),
        down_color_thick.1,
        down_color_thick.0,
    );
    volume
}
