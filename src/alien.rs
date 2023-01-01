use crate::dot_map::DotMap;
use macroquad::prelude::*;

pub struct Alien {
    // リファレンスエイリアンの座標
    ref_alien_pos: IVec2,
    // リファレンスエイリアンの現在位置へ動く一つ前の位置
    pre_ref_alien_pos: IVec2,
    // 描画するスプライト
    show_sprite: bool,
    // スプライトのリスト
    sprite_list: Vec<Vec<u8>>,
    // 描画処理対象のインデックス番号
    i_cursor_alien: usize,
    // エイリアンの移動量
    move_delta: IVec2,
}

impl Alien {
    pub fn new(
        // 下2列のエイリアンのスプライト
        low_sprite0: Vec<u8>,
        low_sprite1: Vec<u8>,
        // 中2列のエイリアンのスプライト
        middle_sprite0: Vec<u8>,
        middle_sprite1: Vec<u8>,
        // 上1列のエイリアンのスプライト
        high_sprite0: Vec<u8>,
        high_sprite1: Vec<u8>,
    ) -> Self {
        let mut sprite_list = Vec::new();
        sprite_list.push(low_sprite0);
        sprite_list.push(low_sprite1);
        sprite_list.push(middle_sprite0);
        sprite_list.push(middle_sprite1);
        sprite_list.push(high_sprite0);
        sprite_list.push(high_sprite1);
        Alien {
            ref_alien_pos: IVec2::new(0, 0),
            pre_ref_alien_pos: IVec2::new(0, 0),
            show_sprite: true,
            sprite_list,
            i_cursor_alien: 0,
            move_delta: IVec2::new(2, 0),
        }
    }
    // エイリアンを初期位置に配置
    pub fn init_alien(&mut self) {
        self.ref_alien_pos = IVec2::new(24, 12 * 8);
        self.pre_ref_alien_pos = self.ref_alien_pos;
    }
    pub fn update(&mut self, dot_map: &mut DotMap) {
        self.array_sprite(dot_map);

        // 処理対象カーソルを進める
        self.i_cursor_alien = if self.i_cursor_alien < 54 {
            self.i_cursor_alien + 1
        } else {
            // 一巡後、エイリアンのどれかが両側の折り返し地点に到達していたら反転する
            if self.check_bump_side(dot_map) {
                self.move_delta = IVec2::new(-1 * self.move_delta.x, 8);
            } else {
                // 折り返しが終わったらdyは0にする
                self.move_delta.y = 0;
            }
            // 一巡したら描画するスプライトを切り替える
            self.show_sprite = !self.show_sprite;
            // 移動前のリファレンスエイリアンの座標を保存する
            self.pre_ref_alien_pos = self.ref_alien_pos;
            // リファレンスエイリアンを移動させる
            self.ref_alien_pos += self.move_delta;
            0
        }
    }
    fn array_sprite(&mut self, dot_map: &mut DotMap) {
        let i = self.i_cursor_alien;
        let alien_pos = Alien::ret_alien_pos(i, self.pre_ref_alien_pos);
        let sprite_type: usize = if self.show_sprite { 0 } else { 1 };
        let sprite = &self.sprite_list[2 * Alien::ret_alien_type(i) + sprite_type];
        let width = sprite.len();

        // エイリアンをドットマップに描画(縦方向のバイト境界はまたがない)
        // 前回描画した移動前の部分を0で消す
        let char_y = (alien_pos.y / 8) as usize;
        for dx in 0..width {
            dot_map.map[char_y][alien_pos.x as usize + dx] = 0;
        }
        // 移動後を描画する
        let alien_pos = Alien::ret_alien_pos(i, self.ref_alien_pos);
        let char_y = (alien_pos.y / 8) as usize;
        for dx in 0..width {
            dot_map.map[char_y][alien_pos.x as usize + dx] = sprite[dx];
        }
    }
    // 何かの物体が両側の折り返し地点に到達していたら真を返す
    fn check_bump_side(&self, dot_map: &DotMap) -> bool {
        // 判定する壁の高さはUFOの下からプレイヤーの上まで
        for char_y in 2..23 {
            // 左右どちらかの壁のドットに何かが存在したら
            if dot_map.map[char_y][9] != 0 || dot_map.map[char_y][213] != 0 {
                return true;
            }
        }
        false
    }
    // リファレンスエイリアンの座標とインデックス番号から座標を返す
    fn ret_alien_pos(i: usize, ref_pos: IVec2) -> IVec2 {
        let dx = i as i32 % 11;
        let dy = i as i32 / 11;
        IVec2::new(ref_pos.x + 16 * dx, ref_pos.y - 16 * dy)
    }
    // インデックス番号から、下2段は0、中2段は1、上1段は2を返す
    fn ret_alien_type(i: usize) -> usize {
        let row = i / 11;
        match row {
            0 | 1 => 0,
            2 | 3 => 1,
            4 => 2,
            _ => panic!("エイリアンを指すインデックス番号が不正です。"),
        }
    }
}
