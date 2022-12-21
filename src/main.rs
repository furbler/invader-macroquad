use dot_data::DotShape;
use macroquad::prelude::*;
use macroquad::window;
use std::error::Error;

mod dot_data;
use crate::dot_data::dot_data;

struct Bullet {
    width: f32,    // 描画サイズの幅 [pixel]
    height: f32,   // 描画サイズの高さ [pixel]
    pre_pos: Vec2, // 前回描画時の位置
    pos: Vec2,     // 中心位置
    live: bool,    // 弾が存在しているか否か
    texture: Texture2D,
}

struct Player {
    width: f32,    // 描画サイズの幅 [pixel]
    height: f32,   // 描画サイズの高さ [pixel]
    pre_pos: Vec2, // 前回描画時の位置
    pos: Vec2,     // 中心位置
    texture: Texture2D,
    bullet: Bullet, // 弾
}
impl Player {
    fn update(&mut self) {
        // 弾が存在していたら
        if self.bullet.live {
            // 弾の移動処理
            self.bullet.pos.y -= 12.;
            // 弾が画面上の外側に行ったら
            if self.bullet.pos.y < 0. {
                // 弾を消す
                self.bullet.live = false;
            }
        }
        // 発射ボタンが押された場合(スペース、上矢印、Enter)
        if is_key_down(KeyCode::Space) || is_key_down(KeyCode::Enter) {
            // 弾が画面上に存在しない場合
            if !self.bullet.live {
                self.bullet.pos.x = self.pos.x;
                self.bullet.pos.y = self.pos.y - self.height;
                self.bullet.live = true;
            }
        }

        // プレイヤー移動範囲制限
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
    fn draw(&mut self) {
        // プレイヤー
        draw_texture_ex(
            self.texture,
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.width, self.height)),
                ..Default::default()
            },
        );

        // プレイヤーの弾が画面上に存在する時のみ描画する
        if self.bullet.live {
            draw_texture_ex(
                self.bullet.texture,
                self.bullet.pos.x - self.bullet.width / 2.,
                self.bullet.pos.y - self.bullet.height / 2.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(self.bullet.width, self.bullet.height)),
                    ..Default::default()
                },
            );
        }
        self.pre_pos = self.pos;
        self.bullet.pre_pos = self.bullet.pos;
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    window::request_new_screen_size(108., 108.);

    // キャラクターのドット絵読み込み
    let player_data = dot_data("player");
    let bullet_player_data = dot_data("bullet_player");
    // ここで実際の描画サイズと色を指定する
    let mut player = Player {
        width: player_data.width as f32 * 3.,
        height: player_data.height as f32 * 3.,
        pos: Vec2::new(screen_width() / 2., screen_height() - 120.),
        pre_pos: Vec2::new(screen_width() / 2., screen_height() - 120.),
        texture: dot_map2texture(&player_data),
        bullet: Bullet {
            width: bullet_player_data.width as f32 * 3.,
            height: bullet_player_data.height as f32 * 3.,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            texture: dot_map2texture(&player_data),
        },
    };

    loop {
        clear_background(BLACK);
        // プレイヤー下の横線
        draw_line(
            0.,
            screen_height() - 50.,
            screen_width(),
            screen_height() - 50.,
            1.,
            RED,
        );
        player.update();
        // プレイヤー表示
        player.draw();
        next_frame().await
    }
}

// ドットデータをテクスチャデータに変換
fn dot_map2texture(chara: &DotShape) -> Texture2D {
    let texture = Texture2D::from_rgba8(chara.width, chara.height, &chara.create_color_dot_map());
    texture.set_filter(FilterMode::Nearest);
    texture
}
// ウィンドウサイズを指定
fn window_conf() -> Conf {
    Conf {
        window_title: "invader-macroquad".to_owned(),
        window_width: 208 * 3,
        window_height: 208 * 3,
        ..Default::default()
    }
}
