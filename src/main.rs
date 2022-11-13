use dot_data::DotShape;
use macroquad::prelude::*;
use std::error::Error;

mod dot_data;
use crate::dot_data::dot_data;

struct Character {
    width: f32,  // 描画サイズの幅 [pixel]
    height: f32, // 描画サイズの高さ [pixel]
    pos: Vec2,
    texture: Texture2D,
}
#[macroquad::main("invader-macroquad")]
async fn main() -> Result<(), Box<dyn Error>> {
    // キャラクターのドット絵
    let player_data = dot_data("player");
    let crab_down_data = dot_data("crab_down");
    let crab_banzai_data = dot_data("crab_banzai");
    // ここで実際の描画サイズと色を指定する
    let mut player = Character {
        width: player_data.width as f32 * 3.,
        height: player_data.height as f32 * 3.,
        pos: Vec2::new(screen_width() / 2., screen_height() - 120.),
        texture: dot_map2texture("TURQUOISE", player_data),
    };
    let mut crab_down = Character {
        width: crab_down_data.width as f32 * 3.,
        height: crab_down_data.height as f32 * 3.,
        pos: Vec2::new(screen_width() / 3., screen_height() / 3.),
        texture: dot_map2texture("PURPLE", crab_down_data),
    };
    let mut crab_banzai = Character {
        width: crab_banzai_data.width as f32 * 3.,
        height: crab_banzai_data.height as f32 * 3.,
        pos: Vec2::new(screen_width() * 0.7, screen_height() / 3.),
        texture: dot_map2texture("PURPLE", crab_banzai_data),
    };

    loop {
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            player.pos.x -= 5.;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            player.pos.x += 5.;
        }
        // 背景色描画
        clear_background(BLACK);
        // プレイヤーの下の横線
        draw_line(
            0.,
            screen_height() - 50.,
            screen_width(),
            screen_height() - 50.,
            3.,
            RED,
        );

        // 描画
        draw_texture_ex(
            player.texture,
            player.pos.x,
            player.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(player.width, player.height)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            crab_down.texture,
            crab_down.pos.x,
            crab_down.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(crab_down.width, crab_down.height)),
                ..Default::default()
            },
        );

        draw_texture_ex(
            crab_banzai.texture,
            crab_banzai.pos.x,
            crab_down.pos.y,
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
