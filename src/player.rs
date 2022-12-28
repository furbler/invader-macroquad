use crate::dot_map::DotMap;
use macroquad::prelude::*;
pub struct Bullet {
    pos: IVec2,      // 左上位置
    pre_pos: IVec2,  // 前回描画時の位置
    live: bool,      // 弾が存在しているか否か
    sprite: Vec<u8>, // 左側から縦8ピクセルずつを8bitのベクタで表す
}

impl Bullet {
    pub fn new(sprite: Vec<u8>) -> Self {
        Bullet {
            pos: IVec2::new(0, 0),
            pre_pos: IVec2::new(0, 0),
            live: false,
            sprite,
        }
    }
    // 弾を発射
    fn fire(&mut self, x: i32, y: i32) {
        // 弾が画面上に存在しない場合
        if !self.live {
            self.pos = IVec2::new(x, y);
            self.live = true;
        }
    }
    pub fn update(&mut self, player_pos: IVec2, dot_map: &mut DotMap) {
        // 弾が存在していたら
        if self.live {
            // 弾の移動処理
            self.pos.y -= 3;
            // 弾が画面上部に行ったら
            if self.pos.y < 0 {
                self.pos.y = 0;
                // 弾を消す
                self.live = false;
                self.erase(dot_map);
            }
        } else {
            // 発射ボタンが押された場合(スペース、Enter)
            if is_key_down(KeyCode::Space) || is_key_down(KeyCode::Enter) {
                self.fire(player_pos.x + 7, player_pos.y - 8);
            }
        }
    }

    // プレイヤーの弾をドットマップに描画(縦方向のバイト境界をまたぐ可能性有り)
    pub fn array_sprite(&mut self, dot_map: &mut DotMap) {
        if !self.live {
            return;
        }
        self.erase(dot_map);
        // 移動後描画する
        let char_y = (self.pos.y / 8) as usize;
        let char_offset_bit = (self.pos.y % 8) as u8;
        // 1にしたいbitには1、透過部分には0をおく
        let bit_mask: u8 = self.sprite[0] << char_offset_bit;
        dot_map.map[char_y][self.pos.x as usize] |= bit_mask;
        // 下側にはみ出した部分
        let bit_mask = self.sprite[0] >> (7 - char_offset_bit);
        dot_map.map[char_y + 1][self.pos.x as usize] |= bit_mask;

        self.pre_pos = self.pos;
    }
    // 描画された弾を透過ありで消す
    fn erase(&self, dot_map: &mut DotMap) {
        // 前回描画した部分を0で消す
        let char_y = (self.pre_pos.y / 8) as usize;
        let char_offset_bit = (self.pre_pos.y % 8) as u8;
        // 0クリアしたいbitには0、透過部分には1をおく
        //上側
        let bit_mask: u8 = !(self.sprite[0] << char_offset_bit);
        dot_map.map[char_y][self.pre_pos.x as usize] &= bit_mask;
        // 下側にはみ出した部分
        let bit_mask = !(self.sprite[0] >> (7 - char_offset_bit));
        dot_map.map[char_y + 1][self.pre_pos.x as usize] &= bit_mask;
    }
}

pub struct Player {
    width: i32,            // 描画サイズの幅
    canvas_dot_width: i32, // キャンバスのドット幅
    pub pos: IVec2,        // 左上位置
    pre_pos: IVec2,        // 前回描画時の位置
    sprite: Vec<u8>,       // 左側から縦8ピクセルずつを8bitのベクタで表す
}
impl Player {
    pub fn new(canvas_dot_width: i32, canvas_dot_height: i32, sprite: Vec<u8>) -> Self {
        Player {
            width: sprite.len() as i32,
            canvas_dot_width,
            pos: IVec2::new(8, canvas_dot_height - 8 * 3),
            pre_pos: IVec2::new(8, canvas_dot_height - 8 * 3),
            sprite,
        }
    }
    pub fn update(&mut self) {
        // プレイヤー移動範囲制限
        if 7 < self.pos.x && (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) {
            // 左に移動
            self.pos.x -= 1;
        }
        if self.pos.x + self.width < self.canvas_dot_width - 7
            && (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right))
        {
            // 右に移動
            self.pos.x += 1;
        }
    }
    // プレイヤーをドットマップに描画(縦方向のバイト境界はまたがない)
    pub fn array_sprite(&mut self, dot_map: &mut DotMap) {
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
