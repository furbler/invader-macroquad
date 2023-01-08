use crate::dot_map;
use macroquad::prelude::*;
use std::io::Write;

// 1文字8ピクセル分がいくつ入るか
const CHAR_HEIGHT: i32 = 26;
// ドット単位の大きさ
const DOT_HEIGHT: i32 = 8 * CHAR_HEIGHT;
// 最終的に表示されるディスプレイの大きさ
// 上のスコア表示用の4文字分 + 下の残機表示用の1文字分を加える
const ALL_DOT_HEIGHT: i32 = DOT_HEIGHT + 8 * 5;
pub struct BottomArea {
    // G
    display_width: f32,
    player_texture: Texture2D,
    exploding_player_texture: Texture2D,
}

impl BottomArea {
    pub fn new(player_sprite: &Vec<u8>) -> Self {
        let (player_texture, exploding_player_texture) = player_sprite2texture(&player_sprite);
        BottomArea {
            display_width: player_sprite.len() as f32,
            player_texture,
            exploding_player_texture,
        }
    }
    pub fn draw(&self, player_life_num: i32, player_exploding: bool, scale: i32) {
        let text = &format!("{}", player_life_num);
        let font_size = (14 * scale) as f32;
        let font_color;
        let texture;
        if player_exploding {
            // プレイヤーの爆発中は赤色にする
            font_color = Color::new(0.82, 0., 0., 1.00);
            texture = self.exploding_player_texture;
        } else {
            // 通常色
            font_color = Color::new(0.27, 0.78, 0.82, 1.00);
            texture = self.player_texture;
        };
        // 残機の数を表示
        // 指定座標は文字の左下
        draw_text(
            text,
            (8 * scale) as f32,
            ((ALL_DOT_HEIGHT - 1) * scale) as f32,
            font_size,
            font_color,
        );
        // 残機-1の数だけプレイヤーの画像を並べる:w
        let mut x = (24 * scale) as f32;
        let dx = (2 * 8 * scale) as f32;
        for _ in 0..player_life_num - 1 {
            draw_texture_ex(
                texture,
                x,
                ((ALL_DOT_HEIGHT - 8) * scale) as f32,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        self.display_width * scale as f32,
                        (8 * scale) as f32,
                    )),
                    ..Default::default()
                },
            );
            x += dx;
        }
    }
}

// 返り値はプレイヤーのTexture2D(通常色, プレイヤー爆発時の色)
fn player_sprite2texture(player_sprite: &Vec<u8>) -> (Texture2D, Texture2D) {
    let player_rgba = player_sprite2color_bytes(&player_sprite, false);
    let player_exploding_rgba = player_sprite2color_bytes(&player_sprite, true);
    (
        player_rgba2texture(player_rgba, player_sprite.len()),
        player_rgba2texture(player_exploding_rgba, player_sprite.len()),
    )
}

// プレイヤーのスプライトを1ピクセル4バイトのrgbaに変換し、u8のベクタにまとめる
fn player_sprite2color_bytes(player_sprite: &Vec<u8>, player_exploding: bool) -> Vec<u8> {
    let mut color_bytes: Vec<u8> = Vec::new();
    for bit in 0..8 {
        for pos_x in 0..player_sprite.len() {
            if player_sprite[pos_x] & (1 << bit) == 0 {
                // 背景を透過する
                color_bytes.write(&[0, 0, 0, 0]).unwrap();
            } else {
                if player_exploding {
                    // プレイヤーが爆発中はすべて赤にする
                    color_bytes
                        .write(&dot_map::set_color(dot_map::Color::Red))
                        .unwrap();
                } else {
                    // 通常色
                    color_bytes
                        .write(&dot_map::set_color(dot_map::Color::Turquoise))
                        .unwrap();
                }
            }
        }
    }

    color_bytes
}

// プレイヤーのRGBAデータをテクスチャデータに変換
fn player_rgba2texture(rgba: Vec<u8>, width: usize) -> Texture2D {
    let texture = Texture2D::from_rgba8(width as u16, 8, &rgba);
    texture.set_filter(FilterMode::Nearest);
    texture
}
