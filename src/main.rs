use alien::Alien;
use dot_map::DotMap;
use macroquad::{
    audio::{load_sound, Sound},
    prelude::*,
};
use pause::draw_pause;
use player::{Bullet, Player};
use std::error::Error;
use ufo::Ufo;

mod alien;
mod array_sprite;
mod bottom_area;
mod canvas;
mod dot_map;
mod pause;
mod player;
mod sprite;
mod ufo;

#[derive(PartialEq)]
enum Scene {
    Title,
    Play,
    Pause,
    LaunchGame(i32),
    LaunchStage(i32),
    ResetStage,
    Gameover(i32),
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
        load_se_file("audio/player_explosion.wav").await,
    );
    let mut player_bullet = Bullet::new(
        bullet_player_data.create_dot_map(),
        player_bullet_explosion_data.create_dot_map(),
        load_se_file("audio/shoot.wav").await,
    );
    let mut ufo = Ufo::new(
        ufo_data.create_dot_map(),
        ufo_explosion_data.create_dot_map(),
        load_se_file("audio/ufo_flying.wav").await,
        load_se_file("audio/ufo_explosion.wav").await,
    );
    let shield = shield_data.create_dot_map();

    let mut alian_se = Vec::new();
    alian_se.push(load_se_file("audio/fastinvader1.wav").await);
    alian_se.push(load_se_file("audio/fastinvader2.wav").await);
    alian_se.push(load_se_file("audio/fastinvader3.wav").await);
    alian_se.push(load_se_file("audio/fastinvader4.wav").await);
    let mut alien = Alien::new(
        octopus_open_data.create_dot_map(),
        octopus_close_data.create_dot_map(),
        crab_banzai_data.create_dot_map(),
        crab_down_data.create_dot_map(),
        squid_open_data.create_dot_map(),
        squid_close_data.create_dot_map(),
        alien_explosion_data.create_dot_map(),
        alian_se,
        load_se_file("audio/invader_killed.wav").await,
    );
    let mut alien_bullets = alien::BulletManage::new(alien_bullet_explosion_data.create_dot_map());

    // 真の場合、画面全体を赤色にする
    let mut player_exploding = false;
    // ステージの面数
    let mut stage = 1;
    // 起動直後はタイトル画面から始める
    let mut scene = Scene::Title;
    // 全体の音量(0〜100)
    let mut volume = 30;
    loop {
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

        match scene {
            Scene::Title => {
                if is_key_pressed(KeyCode::Enter) {
                    scene = Scene::LaunchGame(10);
                    // すべて消す
                    map.all_clear();
                }
                // 画面全体を背景色(黒)クリア
                clear_background(BLACK);
                draw_title();
            }
            Scene::Play => {
                // Escキーが押されていたらポーズ
                if is_key_pressed(KeyCode::Escape) {
                    scene = Scene::Pause;
                }
                // 更新処理
                player.set_se_volume(volume);
                player.update(&mut map);
                player_bullet.set_se_volume(volume);
                player_bullet.update(&mut map, &mut player, &mut ufo, &mut alien);

                ufo.set_se_volume(volume);
                ufo.update(&mut map, player_bullet.fire_cnt);

                alien.set_se_volume(volume);
                alien.update(&mut map, player_exploding);
                alien_bullets.update(&mut map, &mut player, &mut alien);
                // エイリアンが全滅したら
                if alien.live_num <= 0 {
                    // 次のステージへ進む
                    scene = Scene::LaunchStage(120);
                }
                // プレイヤーの残機が0またはエイリアンがプレイヤーの高さまで侵攻したら
                if player.life <= 0 || alien.invaded() {
                    // ゲームオーバー
                    scene = Scene::Gameover(120);
                    // 音を止める
                    ufo.reset();
                    if alien.invaded() {
                        // プレイヤーの高さに降りてきた個体を描く
                        alien.update(&mut map, player_exploding);
                        // エイリアンに侵攻されていたら爆発を起こす
                        player.remove(&mut map);
                    };
                }
                // プレイヤーが爆発中は画面全体を赤にする
                player_exploding = if player.explosion_cnt == None {
                    false
                } else {
                    true
                };
            }
            Scene::ResetStage => {
                // ゲーム開始、ステージ開始時共通
                scene = Scene::Play;
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
                alien.reset(stage);
                alien_bullets.reset();
                ufo.reset();
            }
            Scene::LaunchGame(cnt) => {
                // 一定時間経過したらゲーム開始
                if cnt < 0 {
                    scene = Scene::ResetStage;

                    stage = 1;
                    player.reset_all();
                    player_bullet.reset_all();
                } else {
                    scene = Scene::LaunchGame(cnt - 1);
                }
            }
            Scene::LaunchStage(cnt) => {
                // 一定時間経過したら次のステージ開始
                if cnt < 0 {
                    scene = Scene::ResetStage;

                    stage += 1;
                    player.reset_stage();
                    player_bullet.reset_stage();
                } else {
                    scene = Scene::LaunchStage(cnt - 1);
                }
            }
            Scene::Gameover(cnt) => {
                // 一定時間経過したらタイトル画面に戻る
                if cnt < 0 {
                    scene = Scene::Title;
                } else {
                    scene = Scene::Gameover(cnt - 1);
                    // プレイヤーを爆発させる
                    if let Some(cnt) = player.explosion_cnt {
                        if cnt <= player.const_max_explosion_cnt {
                            player.update(&mut map);
                        }
                    }
                }
                draw_gameover_message();
            }
            Scene::Pause => {
                // Escキーが押されていたらポーズ解除
                if is_key_pressed(KeyCode::Escape) {
                    scene = Scene::Play;
                }
                volume = draw_pause(volume);
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
        screen_width() / 2. - str_size.width / 2.,
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
        screen_width() / 2. - str_size.width / 2.,
        270.,
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
// ゲームオーバー表示
fn draw_gameover_message() {
    let text = "Game over";
    let font_size = 120.;
    let str_size = measure_text(text, None, font_size as _, 1.0);
    // 指定座標は文字の左下
    draw_text(
        text,
        screen_width() / 2. - str_size.width / 2.,
        190.,
        font_size,
        RED,
    );
}

async fn load_se_file(path: &str) -> Sound {
    load_sound(path).await.unwrap()
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
