use dot_data::DotShape;
use macroquad::prelude::*;
use std::error::Error;

mod dot_data;
use crate::dot_data::dot_data;

struct Player {
    width: f32,  // 描画サイズの幅 [pixel]
    height: f32, // 描画サイズの高さ [pixel]
    pos: Vec2,   // 中心位置
    texture: Texture2D,
}
impl Player {
    // プレイヤー移動
    fn update(&mut self) {
        if 0. < self.pos.x - self.width / 2.
            && (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left))
        {
            self.pos.x -= 5.;
        }

        if self.pos.x + self.width / 2. < screen_width()
            && (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right))
        {
            self.pos.x += 5.;
        }
    }
    fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.width, self.height)),
                ..Default::default()
            },
        )
    }
}

struct Enemy {
    width: f32,                // 描画サイズの幅 [pixel]
    height: f32,               // 描画サイズの高さ [pixel]
    pos: Vec2,                 // 中心位置
    switched_time: f64,        // 最後に画像切り替えした時間
    select_texture: bool,      // どちらの状態の画像を表示するか
    first_texture: Texture2D,  // 状態1
    second_texture: Texture2D, // 状態2
}
impl Enemy {
    fn draw(&mut self) {
        let current_time = get_time();
        // 表示画像切り替えから一定時間経過していたら
        if current_time - self.switched_time > 1. {
            // 表示画像切り替え
            self.select_texture = !self.select_texture;
            self.switched_time = current_time;
        }
        let texture;
        if self.select_texture {
            texture = self.first_texture;
        } else {
            texture = self.second_texture;
        }

        draw_texture_ex(
            texture,
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.width, self.height)),
                ..Default::default()
            },
        )
    }
}

#[macroquad::main("invader-macroquad")]
async fn main() -> Result<(), Box<dyn Error>> {
    // キャラクターのドット絵
    let player_data = dot_data("player");
    let crab_down_data = dot_data("crab_down");
    let crab_banzai_data = dot_data("crab_banzai");
    // ここで実際の描画サイズと色を指定する
    let mut player = Player {
        width: player_data.width as f32 * 3.,
        height: player_data.height as f32 * 3.,
        pos: Vec2::new(screen_width() / 2., screen_height() - 120.),
        texture: dot_map2texture("TURQUOISE", player_data),
    };
    let mut crab = Enemy {
        width: crab_down_data.width as f32 * 3.,
        height: crab_down_data.height as f32 * 3.,
        pos: Vec2::new(screen_width() / 3., screen_height() / 3.),
        switched_time: get_time(),
        select_texture: true,
        first_texture: dot_map2texture("PURPLE", crab_banzai_data),
        second_texture: dot_map2texture("PURPLE", crab_down_data),
    };

    loop {
        player.update();
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
        player.draw();
        crab.draw();

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
