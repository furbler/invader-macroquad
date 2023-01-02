use crate::alien::Alien;
use crate::dot_map::DotMap;
use crate::ufo::Ufo;
use macroquad::prelude::*;
// プレイヤーの弾のスピード
const PLAYER_BULLET_DELTA: i32 = 4;

pub struct Bullet {
    pos: IVec2,                  // 左上位置
    live: bool,                  // 弾が存在しているか否か
    explosion_effect_show: bool, // 爆発エフェクトを表示するならば真
    ban_fire_cnt: Option<i32>,   // 射撃禁止状態の残りカウント
    pub fire_cnt: i32,           // ステージ開始からの累計射撃数
    sprite: Vec<u8>,             // 左側から縦8ピクセルずつを8bitのベクタで表す
    explosion_sprite: Vec<u8>,   // 爆発画像
}

impl Bullet {
    pub fn new(sprite: Vec<u8>, explosion_sprite: Vec<u8>) -> Self {
        Bullet {
            pos: IVec2::new(0, 0),
            live: false,
            explosion_effect_show: false,
            ban_fire_cnt: None,
            fire_cnt: 0,
            sprite,
            explosion_sprite,
        }
    }
    // 弾を発射
    fn fire(&mut self, x: i32, y: i32) {
        // 弾が画面上に存在しない場合
        if !self.live {
            self.pos = IVec2::new(x, y);
            self.live = true;
            self.fire_cnt += 1;
        }
    }
    pub fn update(
        &mut self,
        dot_map: &mut DotMap,
        player_pos: IVec2,
        ufo: &mut Ufo,
        alien: &mut Alien,
    ) {
        // 弾が存在していたら
        if self.live {
            // 前回の弾を消す
            self.erase(dot_map);
            // 弾の移動
            self.pos.y -= PLAYER_BULLET_DELTA;
            // 弾が画面上部に行ったら
            if self.pos.y < 0 {
                self.pos.y = 0;
                // 弾を消す
                self.live = false;
                self.ban_fire_cnt = Some(15);
                self.explosion_effect_show = true;
                // 自身のx座標が爆発エフェクトの中心になるようずらす
                self.pos.x = self.pos.x - self.explosion_sprite.len() as i32 / 2;
            } else {
                // 移動後の弾の部分に何か物体が存在したら
                if self.collision(dot_map) {
                    // 何かに衝突したので弾を消す
                    self.live = false;
                    self.ban_fire_cnt = Some(15);
                    // 爆発エフェクトを表示する
                    self.explosion_effect_show = true;
                    // 衝突したのがUFOだった場合
                    if self.pos.y / 8 < 2 {
                        // UFOの爆発エフェクト表示中でなければ
                        if ufo.explosion.show_cnt == None {
                            // UFOの撃破
                            ufo.hit_player_bullet(dot_map);
                        }
                        // 爆発エフェクトは表示しない
                        self.explosion_effect_show = false;
                    } else if self.pos.y <= alien.ref_alien_pos.y + 6 {
                        // 衝突したのがUFO(の高さ)より下かつ、リファレンスエイリアンより上だった場合のみ
                        // エイリアンに当たっていた場合
                        if let Some(i) = alien.ret_alien_index(self.pos) {
                            alien.remove(dot_map, i);
                            // 爆発エフェクトは表示しない
                            self.explosion_effect_show = false;
                        }
                    }
                    // 自身のx座標が爆発エフェクトの中心になるようずらす
                    self.pos.x = self.pos.x - 5;
                    // 少し上にずらす
                    self.pos.y -= 2;
                }
            }
        } else {
            // 弾が画面上に無く、射撃可能状態で、かつ発射ボタンが押された場合(スペース、Enter)
            if self.ban_fire_cnt == None
                && (is_key_down(KeyCode::Space) || is_key_down(KeyCode::Enter))
            {
                self.fire(player_pos.x + 7, player_pos.y - 8);
            }
        }
    }

    // プレイヤーの弾をドットマップに描画(縦方向のバイト境界をまたぐ可能性有り)
    pub fn array_sprite(&mut self, dot_map: &mut DotMap) {
        if let Some(cnt) = self.ban_fire_cnt {
            self.ban_fire_cnt = if cnt < 0 {
                if self.explosion_effect_show {
                    // 爆発エフェクトを消す
                    self.erase_explosion(dot_map);
                }
                None
            } else {
                if self.explosion_effect_show {
                    // 弾の爆発エフェクト表示
                    let char_y = (self.pos.y / 8) as usize;
                    let char_offset_bit = (self.pos.y % 8) as u8;
                    for x in 0..self.explosion_sprite.len() {
                        // 1にしたいbitには1、透過部分には0をおく
                        let bit_mask: u8 = self.explosion_sprite[x] << char_offset_bit;
                        dot_map.map[char_y][self.pos.x as usize + x] |= bit_mask;
                    }
                    if char_offset_bit != 0 {
                        // 下側にはみ出した部分
                        for x in 0..self.explosion_sprite.len() {
                            // 1にしたいbitには1、透過部分には0をおく
                            let bit_mask = self.explosion_sprite[x] >> (8 - char_offset_bit);
                            dot_map.map[char_y + 1][self.pos.x as usize + x] |= bit_mask;
                        }
                    }
                }
                Some(cnt - 1)
            };
        }
        // 弾がなければ何もしない
        if !self.live {
            return;
        }
        // 移動後描画する
        let char_y = (self.pos.y / 8) as usize;
        let char_offset_bit = (self.pos.y % 8) as u8;
        // 1にしたいbitには1、透過部分には0をおく
        let bit_mask: u8 = self.sprite[0] << char_offset_bit;
        dot_map.map[char_y][self.pos.x as usize] |= bit_mask;
        if char_offset_bit != 0 {
            // 下側にはみ出した部分
            let bit_mask = self.sprite[0] >> (8 - char_offset_bit);
            dot_map.map[char_y + 1][self.pos.x as usize] |= bit_mask;
        }
    }
    // 弾の当たり判定
    fn collision(&self, dot_map: &DotMap) -> bool {
        let char_y = (self.pos.y / 8) as usize;
        let offset_bit = (self.pos.y % 8) as u8;
        // 移動した弾の部分のビットマスクを作る
        let bit_mask = self.sprite[0] & !(0b1111_1111 << PLAYER_BULLET_DELTA);
        // ビットがバイトの境界をまたぐときの上下それぞれの判定
        let high = dot_map.map[char_y][self.pos.x as usize] & (bit_mask << offset_bit) != 0;
        let low = if offset_bit != 0 {
            dot_map.map[char_y + 1][self.pos.x as usize] & (bit_mask >> (8 - offset_bit)) != 0
        } else {
            false
        };
        high || low
    }

    // 描画された弾を透過ありで消す
    fn erase(&self, dot_map: &mut DotMap) {
        // 現在位置の弾を0で消す
        let char_y = (self.pos.y / 8) as usize;
        let char_offset_bit = (self.pos.y % 8) as u8;
        // 0クリアしたいbitには0、透過部分には1をおく
        //上側
        let bit_mask: u8 = !(self.sprite[0] << char_offset_bit);
        dot_map.map[char_y][self.pos.x as usize] &= bit_mask;
        if char_offset_bit != 0 {
            // 下側にはみ出した部分
            let bit_mask = !(self.sprite[0] >> (8 - char_offset_bit));
            dot_map.map[char_y + 1][self.pos.x as usize] &= bit_mask;
        }
    }
    fn erase_explosion(&self, dot_map: &mut DotMap) {
        // 弾の爆発エフェクト削除
        let char_y = (self.pos.y / 8) as usize;
        let char_offset_bit = (self.pos.y % 8) as u8;
        for x in 0..self.explosion_sprite.len() {
            // 0にしたいbitには0、透過部分には1をおく
            let bit_mask: u8 = !(self.explosion_sprite[x] << char_offset_bit);
            dot_map.map[char_y][self.pos.x as usize + x] &= bit_mask;
        }
        if char_offset_bit != 0 {
            // 下側にはみ出した部分
            for x in 0..self.explosion_sprite.len() {
                // 0にしたいbitには0、透過部分には1をおく
                let bit_mask = !(self.explosion_sprite[x] >> (8 - char_offset_bit));
                dot_map.map[char_y + 1][self.pos.x as usize + x] &= bit_mask;
            }
        }
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
