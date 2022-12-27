use macroquad::prelude::*;
use std::{error::Error, io::Write};

mod dot_data;

// 1文字8ピクセル分がいくつ入るか
const CHAR_WIDTH: i32 = 28;
const CHAR_HEIGHT: i32 = 26;
// ドット単位の大きさ
const DOT_WIDTH: i32 = 8 * CHAR_WIDTH;
const DOT_HEIGHT: i32 = 8 * CHAR_HEIGHT;

struct DotMap {
    // ドット単位の処理をする範囲(8bit x (上下26 x 左右28))
    // 横8x28、縦26個のu8がある二次元配列
    // 上からy文字目、左からxドット目にあるu8はmap[y][x]
    map: [[u8; DOT_WIDTH as usize]; CHAR_HEIGHT as usize],
}

impl DotMap {
    fn new() -> Self {
        // 0クリアしたドットマップを生成
        DotMap {
            map: [[0; DOT_WIDTH as usize]; CHAR_HEIGHT as usize],
        }
    }
    // 指定したドット単位のY座標のすべてを1にして水平の線を引く
    fn draw_holizon_line(&mut self, y: i32) {
        let y = y as usize;
        let char_pos_y = y / 8;
        let mask_val: u8 = 1 << (y % 8);
        for i in 0..DOT_WIDTH as usize {
            self.map[char_pos_y][i] = self.map[char_pos_y][i] | mask_val;
        }
    }
    // DotMapを1ピクセル4バイトでrgbaを表し、u8のベクタにまとめる
    fn convert_to_color_bytes(&self) -> Vec<u8> {
        let mut color_bytes: Vec<u8> = Vec::new();
        for i_char in 0..CHAR_HEIGHT as usize {
            for bit in 0..8 {
                for pos_x in 0..DOT_WIDTH as usize {
                    if self.map[i_char][pos_x] & (1 << bit) == 0 {
                        color_bytes.write(&[0, 0, 0, 255]).unwrap();
                    } else {
                        // 真っ白だと目に負担があるので少し暗くする
                        color_bytes.write(&[200, 200, 200, 255]).unwrap();
                    }
                }
            }
        }
        color_bytes
    }
}

struct Player {
    width: i32,      // 描画サイズの幅
    pos: IVec2,      // 左上位置
    pre_pos: IVec2,  // 前回描画時の位置
    sprite: Vec<u8>, // 左側から縦8ピクセルずつを8bitのベクタで表す
}
impl Player {
    fn update(&mut self) {
        // プレイヤー移動範囲制限
        if 7 < self.pos.x && (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) {
            // 左に移動
            self.pos.x -= 1;
        }
        if self.pos.x + self.width < DOT_WIDTH - 7
            && (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right))
        {
            // 右に移動
            self.pos.x += 1;
        }
    }
    // プレイヤーをドットマップに描画
    fn array_sprite(&mut self, dot_map: &mut DotMap) {
        // 前回描画した部分を0で消す
        let char_y = (self.pre_pos.y / 8) as usize;
        for dx in 0..self.width as usize {
            dot_map.map[char_y][self.pre_pos.x as usize + dx] = 0;
        }
        // 移動後描画する
        let char_y = (self.pos.y / 8) as usize;
        for dx in 0..self.width as usize {
            dot_map.map[char_y][self.pos.x as usize + dx] = self.sprite[dx];
        }

        self.pre_pos = self.pos;
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    // window::request_new_screen_size(108., 108.);
    let mut map = DotMap::new();
    // キャラクターのドット絵読み込み
    let player_data = dot_data::ret_dot_data("player");
    // ここで実際の描画サイズと色を指定する
    let mut player = Player {
        width: player_data.width,
        pos: IVec2::new(8, DOT_HEIGHT - 8 * 3),
        pre_pos: IVec2::new(8, DOT_HEIGHT - 8 * 3),
        sprite: player_data.create_dot_map(),
    };

    // プレイヤーの下の横線
    map.draw_holizon_line(DOT_HEIGHT - 1);

    loop {
        clear_background(BLACK);
        player.update();
        // プレイヤー
        player.array_sprite(&mut map);

        let rgba = map.convert_to_color_bytes();
        let game_texture = rgba2texture(rgba);
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

// RGBAデータをテクスチャデータに変換
fn rgba2texture(rgba: Vec<u8>) -> Texture2D {
    let texture = Texture2D::from_rgba8(DOT_WIDTH as u16, DOT_HEIGHT as u16, &rgba);
    texture.set_filter(FilterMode::Nearest);
    texture
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
