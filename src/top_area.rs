use crate::array_sprite::array_sprite;
use crate::canvas;
use crate::dot_map::Color;
use crate::dot_map::*;

use macroquad::prelude::*;
use std::io::Write;

pub struct TopArea {
    top: Vec<Vec<u8>>,
    num_sprite: Vec<Vec<u8>>,
}

impl TopArea {
    pub fn new(num_sprite: Vec<Vec<u8>>) -> Self {
        // 0クリアしたドットマップを生成
        TopArea {
            top: vec![vec![0; canvas::TOP_WIDTH as usize]; (canvas::TOP_HEIGHT / 8) as usize],
            num_sprite,
        }
    }
    // すべて消す
    pub fn all_clear(&mut self) {
        self.top = vec![vec![0; canvas::TOP_WIDTH as usize]; (canvas::TOP_HEIGHT / 8) as usize];
    }
    // 上に獲得得点を表示
    pub fn draw_score(&mut self, mut score: i32) {
        let mut score_num = Vec::new();
        for _ in 0..5 {
            score_num.push(score % 10);
            score /= 10;
        }
        let mut pos = IVec2::new(24, 24);
        for i in (0..5).rev() {
            array_sprite(&mut self.top, pos, &self.num_sprite[score_num[i] as usize]);
            pos.x += 8;
        }
    }
    pub fn dot_map2texture(&self, player_exploding: bool) -> Texture2D {
        let rgba = self.convert_to_color_bytes(player_exploding);
        Self::rgba2texture(rgba)
    }

    // DotMapを1ピクセル4バイトでrgbaを表し、u8のベクタにまとめる
    fn convert_to_color_bytes(&self, player_exploding: bool) -> Vec<u8> {
        let mut color_bytes: Vec<u8> = Vec::new();
        for i_char in 0..(canvas::TOP_HEIGHT / 8) as usize {
            for bit in 0..8 {
                for pos_x in 0..canvas::TOP_WIDTH as usize {
                    if self.top[i_char][pos_x] & (1 << bit) == 0 {
                        color_bytes.write(&[0, 0, 0, 255]).unwrap();
                    } else {
                        if player_exploding {
                            // プレイヤーが爆発中はすべて赤にする
                            color_bytes.write(&set_color(Color::Red)).unwrap();
                        } else {
                            color_bytes.write(&set_color(Color::White)).unwrap();
                        }
                    }
                }
            }
        }
        color_bytes
    }
    // RGBAデータをテクスチャデータに変換
    fn rgba2texture(rgba: Vec<u8>) -> Texture2D {
        let texture =
            Texture2D::from_rgba8(canvas::TOP_WIDTH as u16, canvas::TOP_HEIGHT as u16, &rgba);
        texture.set_filter(FilterMode::Nearest);
        texture
    }
}
