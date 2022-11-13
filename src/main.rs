use dot_data::DotData;
use macroquad::prelude::*;
use std::error::Error;

mod dot_data;
use crate::dot_data::dot_data;

struct Character {
    width: f32,
    height: f32,
    texture: Texture2D,
}
#[macroquad::main("invader-macroquad")]
async fn main() -> Result<(), Box<dyn Error>> {
    let player = Character {
        width: 50.,
        height: 50.,
        texture: dot_map2texture(dot_data("player")),
    };
    let crab_down = Character {
        width: 50.,
        height: 50.,
        texture: dot_map2texture(dot_data("crab_down")),
    };
    let crab_banzai = Character {
        width: 50.,
        height: 50.,
        texture: dot_map2texture(dot_data("crab_banzai")),
    };

    loop {
        // 背景色描画
        clear_background(LIGHTGRAY);
        // 描画
        draw_texture_ex(
            player.texture,
            screen_width() / 2.,
            screen_height() - 100.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(player.width, player.height)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            crab_down.texture,
            screen_width() / 3.,
            screen_height() / 3.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(crab_down.width, crab_down.height)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            crab_banzai.texture,
            screen_width() * 0.7,
            screen_height() / 3.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(crab_banzai.width, crab_banzai.height)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}

// ドットデータをテクスチャデータに変換
fn dot_map2texture(chara: DotData) -> Texture2D {
    let texture = Texture2D::from_rgba8(chara.width, chara.height, &chara.dot_map);
    texture.set_filter(FilterMode::Nearest);
    texture
}
