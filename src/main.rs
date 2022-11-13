use dot_data::DotShape;
use macroquad::prelude::*;
use std::error::Error;

mod dot_data;
use crate::dot_data::dot_data;

struct Character {
    width: f32,  // 描画サイズの幅
    height: f32, // 描画サイズの高さ
    texture: Texture2D,
}
#[macroquad::main("invader-macroquad")]
async fn main() -> Result<(), Box<dyn Error>> {
    // キャラクターのドット絵
    let player_data = dot_data("player");
    let crab_down_data = dot_data("crab_down");
    let crab_banzai_data = dot_data("crab_banzai");
    // ここで実際の描画サイズと色を指定する
    let player = Character {
        width: player_data.width as f32 * 3.,
        height: player_data.height as f32 * 3.,
        texture: dot_map2texture("TURQUOISE", player_data),
    };
    let crab_down = Character {
        width: crab_down_data.width as f32 * 3.,
        height: crab_down_data.height as f32 * 3.,
        texture: dot_map2texture("RED", crab_down_data),
    };
    let crab_banzai = Character {
        width: crab_banzai_data.width as f32 * 3.,
        height: crab_banzai_data.height as f32 * 3.,
        texture: dot_map2texture("RED", crab_banzai_data),
    };

    loop {
        // 背景色描画
        clear_background(BLACK);
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
fn dot_map2texture(color: &str, chara: DotShape) -> Texture2D {
    let texture = Texture2D::from_rgba8(
        chara.width,
        chara.height,
        &chara.create_color_dot_map(color),
    );
    texture.set_filter(FilterMode::Nearest);
    texture
}
