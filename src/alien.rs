use crate::dot_map::DotMap;
use macroquad::prelude::*;

pub struct Alien {
    // リファレンスエイリアンの座標
    pub ref_alien_pos: IVec2,
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
    // エイリアンの生存状態
    live: Vec<bool>,
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
            live: vec![true; 55],
        }
    }
    // エイリアンを初期化する
    pub fn init_alien(&mut self) {
        self.ref_alien_pos = IVec2::new(24, 12 * 8);
        self.pre_ref_alien_pos = self.ref_alien_pos;
        self.live = vec![true; 55];
    }
    pub fn update(&mut self, dot_map: &mut DotMap) {
        self.array_sprite(dot_map);

        // 処理対象カーソルを進める
        self.i_cursor_alien += 1;
        while self.i_cursor_alien < 55 {
            if self.live[self.i_cursor_alien] {
                break;
            }
            self.i_cursor_alien += 1;
        }

        if self.i_cursor_alien == 55 {
            self.i_cursor_alien = 0;
            // もう一巡、処理対象カーソルを進める
            while self.i_cursor_alien < 55 {
                if self.live[self.i_cursor_alien] {
                    break;
                }
                self.i_cursor_alien += 1;
            }
            if self.i_cursor_alien == 55 {
                println!("エイリアンは全滅した。");
                return;
            }
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
        }
    }
    fn array_sprite(&mut self, dot_map: &mut DotMap) {
        let i = self.i_cursor_alien;
        // エイリアンのインデックス番号とリファレンスエイリアンの座標から該当エイリアンの座標を計算
        let pre_alien_pos = Alien::ret_alien_pos(i, self.pre_ref_alien_pos);
        // 2種類のエイリアンのスプライトのどちらを描画するか
        let sprite_type: usize = if self.show_sprite { 0 } else { 1 };
        let sprite = &self.sprite_list[2 * Alien::ret_alien_type(i) + sprite_type];
        // 描画するエイリアンの横幅
        let width = sprite.len();

        // エイリアンをドットマップに描画(縦方向のバイト境界はまたがない)
        // 前回描画した移動前の部分を0で消す
        let char_y = (pre_alien_pos.y / 8) as usize;
        for dx in 0..width {
            dot_map.map[char_y][pre_alien_pos.x as usize + dx] = 0;
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
    // インデックス番号で指定されたエイリアンを消す
    pub fn remove(&mut self, dot_map: &mut DotMap, i: usize) {
        self.live[i] = false;
        let width = self.sprite_list[2 * Alien::ret_alien_type(i)].len();
        if self.i_cursor_alien < i {
            // 移動前
            let alien_pos = Alien::ret_alien_pos(i, self.pre_ref_alien_pos);
            let char_y = (alien_pos.y / 8) as usize;
            for dx in 0..width {
                dot_map.map[char_y][alien_pos.x as usize + dx] = 0;
            }
        } else {
            // 移動後
            let alien_pos = Alien::ret_alien_pos(i, self.ref_alien_pos);
            let char_y = (alien_pos.y / 8) as usize;
            for dx in 0..width {
                dot_map.map[char_y][alien_pos.x as usize + dx] = 0;
            }
        }
    }
    // プレイヤーの弾の座標を引数として、エイリアンに当たった場合はそのエイリアンのインデックス番号を返す
    pub fn ret_alien_index(&self, mut pos: IVec2) -> Option<usize> {
        let mut ref_pos = self.ref_alien_pos;
        // リファレンスエイリアン移動時のずれを考慮し、左に2ドットずらす
        ref_pos.x -= 2;
        ref_pos.y += 4;
        // リファレンスエイリアンより左側の場合
        if pos.x < ref_pos.x {
            // エイリアンには当たっていない
            return None;
        }
        // 計算を簡単にするため左下座標にする
        ref_pos.y += 8;
        pos.y += 8;

        let row = (ref_pos.y - pos.y) / 16;
        let column = (ref_pos.x - pos.x).abs() / 16;
        // エイリアンの隊列の外側の場合
        if 11 <= column || 5 <= row {
            // エイリアンには当たっていない
            return None;
        }
        let index = (row * 11 + column) as usize;
        // 該当エイリアンの生存判定
        if self.live[index] {
            Some((row * 11 + column) as usize)
        } else {
            None
        }
    }

    // エイリアンのインデックス番号から座標を返す
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
