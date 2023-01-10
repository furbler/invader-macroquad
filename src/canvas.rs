use macroquad::{
    prelude::Color,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

// 1文字8ピクセル分がいくつ入るか
pub const CHAR_WIDTH: i32 = 28;
pub const CHAR_HEIGHT: i32 = 26;
// ドット単位の大きさ
pub const DOT_WIDTH: i32 = 8 * CHAR_WIDTH;
pub const DOT_HEIGHT: i32 = 8 * CHAR_HEIGHT;
// 最終的に表示されるディスプレイの大きさ
// 幅は変わらない
pub const ALL_DOT_WIDTH: i32 = DOT_WIDTH;
// 上のスコア表示用の4文字分 + 下の残機表示用の1文字分を加える
pub const ALL_DOT_HEIGHT: i32 = DOT_HEIGHT + 8 * 5;
// 1ドットを何ピクセル四方で表示するか(pixel / dot)
pub const SCALE: i32 = 3;

// 指定した色を画面全体の上にかぶせる(alpha値を指定可能)
pub fn draw_screen(color: Color) {
    draw_rectangle(0., 0., screen_width(), screen_height(), color);
}
