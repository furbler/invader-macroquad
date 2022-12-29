use dot_map::DotMap;
use macroquad::prelude::*;
use player::{Bullet, Player};
use std::error::Error;
use ufo::Ufo;

mod dot_map;
mod player;
mod sprite;
mod ufo;

// 1文字8ピクセル分がいくつ入るか
const CHAR_WIDTH: i32 = 28;
const CHAR_HEIGHT: i32 = 26;
// ドット単位の大きさ
const DOT_WIDTH: i32 = 8 * CHAR_WIDTH;
const DOT_HEIGHT: i32 = 8 * CHAR_HEIGHT;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut map = DotMap::new();
    // キャラクターのドットデータ読み込み
    let player_data = sprite::ret_dot_data("player");
    let bullet_player_data = sprite::ret_dot_data("bullet_player");
    if bullet_player_data.width != 1 {
        panic!("プレイヤーの弾の幅は1以外は不正です。");
    }
    let player_bullet_explosion_data = sprite::ret_dot_data("player_bullet_explosion");
    let ufo_data = sprite::ret_dot_data("ufo");
    // 各構造体初期化
    let mut player = Player::new(DOT_WIDTH, DOT_HEIGHT, player_data.create_dot_map());
    let mut bullet = Bullet::new(
        bullet_player_data.create_dot_map(),
        player_bullet_explosion_data.create_dot_map(),
    );
    let mut ufo = Ufo::new(DOT_WIDTH, ufo_data.create_dot_map());

    // プレイヤーの下の横線
    map.draw_holizon_line(DOT_HEIGHT - 1);

    loop {
        // 画面全体を背景色(黒)クリア
        clear_background(BLACK);
        player.update();
        bullet.update(player.pos, &mut map);
        ufo.update(&mut map);
        // プレイヤー
        player.array_sprite(&mut map);
        bullet.array_sprite(&mut map);
        ufo.array_sprite(&mut map);

        let game_texture = map.dot_map2texture();
        draw_texture_ex(
            game_texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        next_frame().await
    }
}

// ウィンドウサイズを指定
fn window_conf() -> Conf {
    Conf {
        window_title: "invader-macroquad".to_owned(),
        window_width: DOT_WIDTH * 3,
        window_height: DOT_HEIGHT * 3,
        ..Default::default()
    }
}
