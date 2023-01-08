use alien::Alien;
use canvas::{ALL_DOT_WIDTH, SCALE};
use dot_map::DotMap;
use macroquad::prelude::*;
use player::{Bullet, Player};
use std::error::Error;
use ufo::Ufo;

mod alien;
mod array_sprite;
mod bottom_area;
mod canvas;
mod dot_map;
mod player;
mod sprite;
mod ufo;

const ALL_PIXEL_WIDTH: i32 = ALL_DOT_WIDTH * SCALE;

#[derive(PartialEq)]
enum Scene {
    Title,
    Play,
    Pause,
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut map = DotMap::new();
    // キャラクターのドットデータ読み込み
    let player_data = sprite::ret_dot_data("player");
    let bullet_player_data = sprite::ret_dot_data("bullet_player");
    if bullet_player_data.width != 1 {
        panic!("プレイヤーの弾の幅は1以外は不正です。");
    }
    let player_explosion_1_data = sprite::ret_dot_data("player_explosion_1");
    let player_explosion_2_data = sprite::ret_dot_data("player_explosion_2");
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
    let player_sprite = player_data.create_dot_map();
    // 画面下部
    let bottom = bottom_area::BottomArea::new(&player_sprite);
    let mut player = Player::new(
        player_sprite,
        player_explosion_1_data.create_dot_map(),
        player_explosion_2_data.create_dot_map(),
    );
    let mut player_bullet = Bullet::new(
        bullet_player_data.create_dot_map(),
        player_bullet_explosion_data.create_dot_map(),
    );
    let mut ufo = Ufo::new(
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

    // スコア、残機などすべて初期化する
    let mut reset_all = true;
    // ステージが進むときの初期化
    let mut reset_stage = false;

    // 真の場合、画面全体を赤色にする
    let mut player_exploding = false;
    // 起動直後はタイトル画面から始める
    let mut scene = Scene::Title;
    loop {
        match scene {
            Scene::Title => {
                if is_key_pressed(KeyCode::Enter) {
                    scene = Scene::Play;
                }
                draw_title();
            }
            Scene::Play => {
                // Escキーが押されていたらポーズ
                if is_key_pressed(KeyCode::Escape) {
                    scene = Scene::Pause;
                }

                // ゲーム開始時とステージ開始時共通の処理
                if reset_all || reset_stage {
                    // すべて消す
                    map.all_clear();
                    // プレイヤーの下の横線
                    map.draw_holizon_line(canvas::DOT_HEIGHT - 1);
                    // シールド配置
                    for i in 0..4 {
                        let gap = (shield_data.width as usize + 23) * i;
                        for dx in 0..shield_data.width as usize {
                            map.map[20][gap + 33 + dx] = shield[dx];
                        }
                        for dx in 0..shield_data.width as usize {
                            map.map[21][gap + 33 + dx] = shield[shield_data.width as usize + dx];
                        }
                    }
                    alien.reset();
                    ufo.reset();
                }
                // ゲーム開始時限定の処理
                if reset_all {
                    player.reset_all();
                    player_bullet.reset_all();
                }
                // ステージ開始時限定の処理
                if reset_stage {
                    player.reset_stage();
                    player_bullet.reset_stage();
                }
                reset_all = false;
                reset_stage = false;

                // 更新処理
                player.update(&mut map);
                player_exploding = if player.explosion_cnt == None {
                    false
                } else {
                    true
                };

                player_bullet.update(&mut map, &mut player, &mut ufo, &mut alien);
                ufo.update(&mut map, player_bullet.fire_cnt);
                alien.update(&mut map, player_exploding);
                alien_bullets.update(&mut map, &mut player, &mut alien);
                // エイリアンが全滅したら
                if alien.live_num <= 0 {
                    // 次のステージへ進む
                    reset_stage = true;
                }
                // プレイヤーの残機が0またはエイリアンがプレイヤーの高さまで侵攻したら
                if player.life <= 0 || alien.invaded() {
                    // ゲームオーバー
                    reset_all = true;
                }
            }
            Scene::Pause => {
                // Escキーが押されていたらポーズ解除
                if is_key_pressed(KeyCode::Escape) {
                    scene = Scene::Play;
                }
            }
        }
        if scene == Scene::Pause || scene == Scene::Play {
            // 画面全体を背景色(黒)クリア
            clear_background(BLACK);
            let game_texture = map.dot_map2texture(player_exploding);
            draw_texture_ex(
                game_texture,
                0.,
                (4 * 8 * canvas::SCALE) as f32,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        (canvas::DOT_WIDTH * canvas::SCALE) as f32,
                        (canvas::DOT_HEIGHT * canvas::SCALE) as f32,
                    )),
                    ..Default::default()
                },
            );
            // 得点表示
            draw_score(player_bullet.score, player_exploding);
            // 残機表示
            bottom.draw(player.life, player_exploding, canvas::SCALE);

            if scene == Scene::Pause {
                draw_pause_message();
            }
        }
        next_frame().await
    }
}

fn draw_title() {
    let text = "Invader";
    let font_size = 120.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        (ALL_PIXEL_WIDTH / 2) as f32 - str_size.width / 2.,
        180.,
        font_size,
        RED,
    );
    let text = "Press Enter";
    let font_size = 60.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        (ALL_PIXEL_WIDTH / 2) as f32 - str_size.width / 2.,
        270.,
        font_size,
        RED,
    );
}

fn draw_pause_message() {
    let text = "Pause";
    let font_size = 120.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        (ALL_PIXEL_WIDTH / 2) as f32 - str_size.width / 2.,
        150.,
        font_size,
        RED,
    );
    let text = "Press Escape key";
    let font_size = 60.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        (ALL_PIXEL_WIDTH / 2) as f32 - str_size.width / 2.,
        240.,
        font_size,
        RED,
    );
    let text = "to resume game";
    let font_size = 60.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        (ALL_PIXEL_WIDTH / 2) as f32 - str_size.width / 2.,
        290.,
        font_size,
        RED,
    );
}

// 上に獲得得点を表示
fn draw_score(score: i32, player_exploding: bool) {
    let text = &format!("{:0>5}", score);
    let font_size = (14 * canvas::SCALE) as f32;
    // プレイヤーの爆発中は赤色にする
    let color = if player_exploding {
        Color::new(0.82, 0., 0., 1.00)
    } else {
        Color::new(0.9, 0.9, 0.9, 1.00)
    };
    // 指定座標は文字の左下
    draw_text(
        text,
        (24 * canvas::SCALE) as f32,
        (32 * canvas::SCALE) as f32,
        font_size,
        color,
    );
}

// ウィンドウサイズを指定
fn window_conf() -> Conf {
    Conf {
        window_title: "invader-macroquad".to_owned(),
        window_width: canvas::ALL_DOT_WIDTH * canvas::SCALE,
        window_height: canvas::ALL_DOT_HEIGHT * canvas::SCALE,
        ..Default::default()
    }
}
