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
    // 描画
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
    move_turn: bool,           // 動くか否か
    select_texture: bool,      // どちらの状態の画像を表示するか
    first_texture: Texture2D,  // 状態1
    second_texture: Texture2D, // 状態2
}
impl Enemy {
    // コンストラクタ
    fn new(first_data: &DotShape, second_data: &DotShape, pos: Vec2, color: &str) -> Self {
        // 引数の2つのドットマップのサイズが異なっていたらエラー
        if first_data.width != second_data.width || first_data.height != second_data.height {
            panic!("２つのドットマップサイズが一致しません。プログラムを終了します。");
        }
        Enemy {
            width: first_data.width as f32 * 3.,
            height: first_data.height as f32 * 3.,
            pos,
            move_turn: false,
            select_texture: true,
            first_texture: dot_map2texture(color, &first_data),
            second_texture: dot_map2texture(color, &second_data),
        }
    }
    fn update(&mut self, move_dir: i32) {
        // 動く順番でない時は何もしない
        if !self.move_turn {
            return;
        }
        // 表示画像切り替え
        self.select_texture = !self.select_texture;
        // 方向を考慮して移動
        self.pos.x += 30. * move_dir as f32;
    }
    // 描画
    fn draw(&mut self) {
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
    // 敵インベーダーの列数と行数
    const COLUMN: usize = 11;

    // キャラクターのドット絵読み込み
    let player_data = dot_data("player");
    let crab_down_data = dot_data("crab_down");
    let crab_banzai_data = dot_data("crab_banzai");
    let octopus_open_data = dot_data("octopus_open");
    let octopus_close_data = dot_data("octopus_close");
    let squid_open_data = dot_data("squid_open");
    let squid_close_data = dot_data("squid_close");
    // ここで実際の描画サイズと色を指定する
    let mut player = Player {
        width: player_data.width as f32 * 3.,
        height: player_data.height as f32 * 3.,
        pos: Vec2::new(screen_width() / 2., screen_height() - 120.),
        texture: dot_map2texture("TURQUOISE", &player_data),
    };
    // 敵インベーダーを入れるリスト
    let mut enemy_list = Vec::new();
    let mut invader_pos = Vec2::new(100., screen_height() - 300.);
    for _i in 0..2 {
        for _k in 0..COLUMN {
            enemy_list.push(Enemy::new(
                &octopus_open_data,
                &octopus_close_data,
                invader_pos,
                "PURPLE",
            ));
            invader_pos.x += 50.;
        }
        invader_pos.x = 100.;
        invader_pos.y -= 50.;
    }
    for _i in 0..2 {
        for _k in 0..COLUMN {
            enemy_list.push(Enemy::new(
                &crab_banzai_data,
                &crab_down_data,
                invader_pos,
                "TURQUOISE",
            ));
            invader_pos.x += 50.;
        }
        invader_pos.x = 100.;
        invader_pos.y -= 50.;
    }
    for _i in 0..COLUMN {
        enemy_list.push(Enemy::new(
            &squid_open_data,
            &squid_close_data,
            invader_pos,
            "GREEN",
        ));
        invader_pos.x += 50.;
    }
    // 一番左下の敵インベーダーから動く
    enemy_list[0].move_turn = true;

    // 敵インベーダーが折り返す境界線の位置
    let left_border = 80.;
    let right_border = screen_width() - 80.;
    // 移動方向(正の値は右、負の値は左に向かう)
    let mut move_dir = 1;
    // 次の移動方向を反転すべきか否か
    let mut move_dir_invert = false;

    loop {
        player.update();
        for enemy in enemy_list.iter_mut() {
            enemy.update(move_dir);
        }
        // 背景色描画
        clear_background(BLACK);
        // プレイヤー下の横線
        draw_line(
            0.,
            screen_height() - 50.,
            screen_width(),
            screen_height() - 50.,
            3.,
            RED,
        );
        // プレイヤー描画
        player.draw();
        // 敵描画
        for enemy in enemy_list.iter_mut() {
            enemy.draw();
        }
        // 移動した敵インベーダーの個体番号を取得
        let mut move_enemy_index = 0;
        for (index, enemy) in enemy_list.iter().enumerate() {
            if enemy.move_turn {
                move_enemy_index = index;
                break;
            }
        }
        // 制限範囲外に出た敵個体がいた場合
        if enemy_list[move_enemy_index].pos.x < left_border
            || right_border < enemy_list[move_enemy_index].pos.x
        {
            // 移動方向反転フラグを立てる
            move_dir_invert = true;
        }
        // 移動する個体を変える
        enemy_list[move_enemy_index].move_turn = false;
        move_enemy_index += 1;
        if move_enemy_index >= enemy_list.len() {
            // 最後の個体だったら、最初の個体に戻る
            enemy_list[0].move_turn = true;
            // 移動方向反転フラグが立っている場合
            if move_dir_invert {
                // 実際に移動方向を反転
                move_dir *= -1;
                move_dir_invert = false;
            }
        } else {
            // 次の個体を動かす
            enemy_list[move_enemy_index].move_turn = true;
        }

        next_frame().await
    }
}

// ドットデータをテクスチャデータに変換
fn dot_map2texture(color: &str, chara: &DotShape) -> Texture2D {
    let texture = Texture2D::from_rgba8(
        chara.width,
        chara.height,
        &chara.create_color_dot_map(color),
    );
    texture.set_filter(FilterMode::Nearest);
    texture
}
