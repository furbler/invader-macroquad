use macroquad::{
    prelude::Color,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};
// 画面の幅(文字単位)
const CHAR_WIDTH: i32 = 28;
// 画面の上部分（スコアなどの表示用）のドット単位の大きさ(28文字x4文字)
pub const TOP_WIDTH: i32 = 8 * CHAR_WIDTH;
pub const TOP_HEIGHT: i32 = 8 * 4;

// メインのゲーム画面のドット単位の大きさ(28文字x26文字)
pub const GAME_WIDTH: i32 = 8 * CHAR_WIDTH;
pub const GAME_HEIGHT: i32 = 8 * 26;
// 画面の上部分（スコアなどの表示用）のドット単位の大きさ(28文字x4文字)
pub const BOTTOM_WIDTH: i32 = 8 * CHAR_WIDTH;
pub const BOTTOM_HEIGHT: i32 = 8 * 2;

// 1ドットを何ピクセル四方で表示するか(pixel / dot)
#[cfg(not(target_arch = "wasm32"))]
pub const SCALE: i32 = 3;
// wasm版は少し小さくする
#[cfg(target_arch = "wasm32")]
pub const SCALE: i32 = 2;

// 指定した色を画面全体の上にかぶせる(alpha値を指定可能)
pub fn draw_screen(color: Color) {
    draw_rectangle(0., 0., screen_width(), screen_height(), color);
}
// ドット座標を最終的なピクセル座標に変換
pub fn dot2pix(dot_scale: i32) -> f32 {
    (dot_scale * SCALE) as f32
}
