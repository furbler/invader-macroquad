use alien::Alien;
use dot_map::DotMap;
use macroquad::prelude::*;
use player::{Bullet, Player};
use std::error::Error;
use ufo::Ufo;

mod alien;
mod array_sprite;
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
    let ufo_explosion_data = sprite::ret_dot_data("ufo_explosion");
    let shield_data = sprite::ret_dot_data("shield");
    let octopus_open_data = sprite::ret_dot_data("octopus_open");
    let octopus_close_data = sprite::ret_dot_data("octopus_close");
    let crab_banzai_data = sprite::ret_dot_data("crab_banzai");
    let crab_down_data = sprite::ret_dot_data("crab_down");
    let squid_open_data = sprite::ret_dot_data("squid_open");
    let squid_close_data = sprite::ret_dot_data("squid_close");
    let alien_explosion_data = sprite::ret_dot_data("alien_explosion");
    let alien_bullet_explosion_data = sprite::ret_dot_data("alien_bullet_explosion");

    // 各構造体初期化
    let mut player = Player::new(DOT_WIDTH, DOT_HEIGHT, player_data.create_dot_map());
    let mut bullet = Bullet::new(
        bullet_player_data.create_dot_map(),
        player_bullet_explosion_data.create_dot_map(),
    );
    let mut ufo = Ufo::new(
        DOT_WIDTH,
        ufo_data.create_dot_map(),
        ufo_explosion_data.create_dot_map(),
    );
    let shield = shield_data.create_dot_map();

    let mut alien = Alien::new(
        octopus_open_data.create_dot_map(),
        octopus_close_data.create_dot_map(),
        crab_banzai_data.create_dot_map(),
        crab_down_data.create_dot_map(),
        squid_open_data.create_dot_map(),
        squid_close_data.create_dot_map(),
        alien_explosion_data.create_dot_map(),
    );
    let mut alien_bullets = alien::BulletManage::new(alien_bullet_explosion_data.create_dot_map());

    alien.init_alien();

    // プレイヤーの下の横線
    map.draw_holizon_line(DOT_HEIGHT - 1);
    for i in 0..4 {
        let gap = (shield_data.width as usize + 23) * i;
        for dx in 0..shield_data.width as usize {
            map.map[20][gap + 33 + dx] = shield[dx];
        }
        for dx in 0..shield_data.width as usize {
            map.map[21][gap + 33 + dx] = shield[shield_data.width as usize + dx];
        }
    }
    loop {
        // 画面全体を背景色(黒)クリア
        clear_background(BLACK);
        // 更新処理
        player.update();
        player.draw(&mut map);

        bullet.update(&mut map, &mut player, &mut ufo, &mut alien);
        bullet.draw(&mut map);

        ufo.update(&mut map, bullet.fire_cnt);
        ufo.draw(&mut map);

        alien.update_draw(&mut map);

        alien_bullets.update(&mut map, &mut player, &mut alien);
        alien_bullets.draw(&mut map);

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
