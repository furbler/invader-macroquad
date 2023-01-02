use crate::dot_map::DotMap;
use macroquad::prelude::*;

// 1文字8ピクセル分がいくつ入るか
const CHAR_HEIGHT: i32 = 26;
// ドット単位の大きさ
const DOT_HEIGHT: i32 = 8 * CHAR_HEIGHT;

enum BulletType {
    Squiggly, // ジグザグ型
    Plunger,  // 十字架型(ピストン型)
    Rolling,  // ねじ型
}

struct Bullet {
    pos: IVec2,
    btype: BulletType,
    live: bool,
    // 種類によらず、サイズは3x8ドット
    sprite: [u8; 3],
}

impl Bullet {
    fn fire(&mut self, pos: IVec2) {
        self.pos = pos;
        self.live = true;
    }
    fn update_squiggly_sprite(&mut self, pos_y: i32) {
        // 0クリア
        for i in 0..3 {
            self.sprite[i] = 0;
        }
        // i = 0..4
        let mut i = (pos_y as usize % 12) / 3;
        let table = [2, 1, 0, 1];
        for y in 1..8 {
            self.sprite[table[i]] |= 1 << y;
            i = (i + 1) % 4;
        }
    }
    fn update_rolling_sprite(&mut self, pos_y: i32) {
        // 真ん中は常に描く
        self.sprite[0] = 0;
        self.sprite[1] = 0b1111_1111;
        self.sprite[2] = 0;

        // i = 0..20
        let i = (pos_y as usize % (20 * 3)) / 3;

        if i < 8 {
            // スラッシュ
            self.sprite[0] = 0b10010000 >> i;
            self.sprite[2] = 0b01001000 >> i;
        } else if 12 <= i {
            // バックスラッシュ
            self.sprite[0] = 0b01001000 >> (i - 12);
            self.sprite[2] = 0b10010000 >> (i - 12);
        }
    }
    fn update_plunger_sprite(&mut self, pos_y: i32) {
        // 真ん中は常に描く
        self.sprite[0] = 0;
        self.sprite[1] = 0b1111_1111;
        self.sprite[2] = 0;
        // i = 0..8
        let i = (pos_y as usize % 24) / 3;

        self.sprite[0] |= 1 << (7 - i);
        self.sprite[2] |= 1 << (7 - i);
    }
    fn update(&mut self, dot_map: &mut DotMap) {
        if !self.live {
            return;
        }
        // 前回の描画を消す
        self.erase(dot_map);
        self.pos.y += 1;
        if DOT_HEIGHT - 1 < self.pos.y + 7 {
            // 赤線に着弾
            // はみださないようにする
            self.pos.y = DOT_HEIGHT - 8;
            self.live = false;
            self.erase(dot_map);
            return;
        }
        match self.btype {
            BulletType::Squiggly => self.update_squiggly_sprite(self.pos.y),
            BulletType::Plunger => self.update_plunger_sprite(self.pos.y),
            BulletType::Rolling => self.update_rolling_sprite(self.pos.y),
        }
    }
    fn array_sprite(&self, dot_map: &mut DotMap) {
        if !self.live {
            return;
        }
        let char_y = (self.pos.y / 8) as usize;
        let char_offset_bit = (self.pos.y % 8) as u8;
        for x in 0..self.sprite.len() {
            // 1にしたいbitには1、透過部分には0をおく
            let bit_mask: u8 = self.sprite[x] << char_offset_bit;
            dot_map.map[char_y][self.pos.x as usize + x] |= bit_mask;
        }
        if char_offset_bit != 0 {
            // 下側にはみ出した部分
            for x in 0..self.sprite.len() {
                // 1にしたいbitには1、透過部分には0をおく
                let bit_mask = self.sprite[x] >> (8 - char_offset_bit);
                dot_map.map[char_y + 1][self.pos.x as usize + x] |= bit_mask;
            }
        }
    }
    // 透過ありで前回の描画を消す
    fn erase(&mut self, dot_map: &mut DotMap) {
        let char_y = (self.pos.y / 8) as usize;
        let char_offset_bit = (self.pos.y % 8) as u8;
        for x in 0..self.sprite.len() {
            // 0にしたいbitには0、透過部分には1をおく
            let bit_mask: u8 = !(self.sprite[x] << char_offset_bit);
            dot_map.map[char_y][self.pos.x as usize + x] &= bit_mask;
        }
        if char_offset_bit != 0 {
            // 下側にはみ出した部分
            for x in 0..self.sprite.len() {
                // 0にしたいbitには0、透過部分には1をおく
                let bit_mask = !(self.sprite[x] >> (8 - char_offset_bit));
                dot_map.map[char_y + 1][self.pos.x as usize + x] &= bit_mask;
            }
        }
    }
}

struct BulletManage {
    // ジグザグ型
    squiggly: Bullet,
    // 十字架型(ピストン型)
    plunger: Bullet,
    // ねじ型
    rolling: Bullet,
}
impl BulletManage {
    fn new() -> Self {
        BulletManage {
            squiggly: Bullet {
                btype: BulletType::Squiggly,
                pos: IVec2::new(0, 0),
                live: false,
                sprite: [0; 3],
            },
            plunger: Bullet {
                btype: BulletType::Plunger,
                pos: IVec2::new(0, 0),
                live: false,
                sprite: [0; 3],
            },
            rolling: Bullet {
                btype: BulletType::Rolling,
                pos: IVec2::new(0, 0),
                live: false,
                sprite: [0; 3],
            },
        }
    }
    fn update(&mut self, dot_map: &mut DotMap) {
        self.plunger.update(dot_map);
        self.squiggly.update(dot_map);
        self.rolling.update(dot_map);
    }
    fn array_sprite(&self, dot_map: &mut DotMap) {
        self.plunger.array_sprite(dot_map);
        self.squiggly.array_sprite(dot_map);
        self.rolling.array_sprite(dot_map);
    }
}

struct Explosion {
    pos: IVec2,
    // 爆発エフェクトのスプライト
    sprite: Vec<u8>,
    //エフェクト表示中はSome(カウント)
    effect_cnt: Option<i32>,
}

impl Explosion {
    // エフェクトを設置する
    fn create_effect(&mut self, dot_map: &mut DotMap, pos: IVec2) {
        self.pos = pos;
        self.effect_cnt = Some(15);

        let char_y = (self.pos.y / 8) as usize;
        for dx in 0..self.sprite.len() {
            dot_map.map[char_y][self.pos.x as usize + dx] = self.sprite[dx];
        }
    }
    fn remove(&mut self, dot_map: &mut DotMap) {
        self.effect_cnt = None;
        let char_y = (self.pos.y / 8) as usize;
        for dx in 0..self.sprite.len() {
            dot_map.map[char_y][self.pos.x as usize + dx] = 0;
        }
    }
    fn update(&mut self, dot_map: &mut DotMap) {
        // エフェクトが表示されていたら
        if let Some(cnt) = self.effect_cnt {
            // カウントが終わったら
            if cnt < 0 {
                // エフェクト削除
                self.remove(dot_map);
                self.effect_cnt = None;
            } else {
                self.effect_cnt = Some(cnt - 1);
            }
        }
    }
}

pub struct Alien {
    // リファレンスエイリアンの座標
    pub ref_alien_pos: IVec2,
    // リファレンスエイリアンの現在位置へ動く一つ前の位置
    pre_ref_alien_pos: IVec2,
    // 描画するスプライト
    show_sprite: bool,
    // スプライトのリスト
    sprite_list: Vec<Vec<u8>>,
    explosion: Explosion,
    // 描画処理対象のインデックス番号
    i_cursor_alien: usize,
    // エイリアンの移動量
    move_delta: IVec2,
    // エイリアンの生存状態
    live: Vec<bool>,
    // 弾
    bullets: BulletManage,
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
        // 爆発エフェクトのスプライト
        explosion_sprite: Vec<u8>,
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
            explosion: Explosion {
                pos: IVec2::new(0, 0),
                sprite: explosion_sprite,
                effect_cnt: None,
            },
            i_cursor_alien: 0,
            move_delta: IVec2::new(2, 0),
            live: vec![true; 55],
            bullets: BulletManage::new(),
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
        self.explosion.update(dot_map);
        self.bullets.update(dot_map);
        self.bullets.array_sprite(dot_map);

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
        if !self.bullets.plunger.live {
            self.bullets.plunger.fire(IVec2::new(
                self.ref_alien_pos.x + 5,
                self.ref_alien_pos.y + 16,
            ));
        }
        if !self.bullets.squiggly.live {
            let pos = Alien::index2pos(1, self.ref_alien_pos);
            self.bullets
                .squiggly
                .fire(IVec2::new(pos.x + 5, pos.y + 16));
        }
        if !self.bullets.rolling.live {
            let pos = Alien::index2pos(2, self.ref_alien_pos);
            self.bullets.rolling.fire(IVec2::new(pos.x + 5, pos.y + 16));
        }
    }
    fn array_sprite(&mut self, dot_map: &mut DotMap) {
        let i = self.i_cursor_alien;
        // エイリアンのインデックス番号とリファレンスエイリアンの座標から該当エイリアンの座標を計算
        let pre_alien_pos = Alien::index2pos(i, self.pre_ref_alien_pos);
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
        let alien_pos = Alien::index2pos(i, self.ref_alien_pos);
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
        let alien_pos;
        // カーソルの前か後かでエイリアンの位置が変わる
        if self.i_cursor_alien < i {
            // カーソルより後ろ
            alien_pos = Alien::index2pos(i, self.pre_ref_alien_pos);
            let char_y = (alien_pos.y / 8) as usize;
            for dx in 0..width {
                dot_map.map[char_y][alien_pos.x as usize + dx] = 0;
            }
        } else {
            // カーソルより前
            alien_pos = Alien::index2pos(i, self.ref_alien_pos);
            let char_y = (alien_pos.y / 8) as usize;
            for dx in 0..width {
                dot_map.map[char_y][alien_pos.x as usize + dx] = 0;
            }
        }
        // 爆発エフェクト描画
        self.explosion.create_effect(dot_map, alien_pos);
    }
    // プレイヤーの弾の座標を引数として、エイリアンに当たった場合はそのエイリアンのインデックス番号を返す
    pub fn pos2index(&self, mut pos: IVec2) -> Option<usize> {
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
    fn index2pos(i: usize, ref_pos: IVec2) -> IVec2 {
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
