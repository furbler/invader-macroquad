use crate::alien::Alien;
use crate::ufo::Ufo;
use crate::{array_sprite::ArraySprite, dot_map::DotMap};
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
            self.explosion_effect_show = false;
        }
    }
    pub fn update(
        &mut self,
        dot_map: &mut DotMap,
        player: &mut Player,
        ufo: &mut Ufo,
        alien: &mut Alien,
    ) {
        // 弾が存在していたら
        if self.live {
            // 前回の弾を消す
            self.erase_shifted(dot_map, self.pos);
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
                        if let Some(i) = alien.pos2index(self.pos) {
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
            // 弾が画面上に無く、射撃可能状態で、プレイヤーが生きていて、かつ発射ボタンが押された場合(スペース、Enter)
            if self.ban_fire_cnt == None
                && player.explosion_cnt == None
                && (is_key_down(KeyCode::Z)
                    || is_key_down(KeyCode::Space)
                    || is_key_down(KeyCode::Enter))
            {
                self.fire(player.pos.x + 7, player.pos.y - 8);
            }
        }
    }

    // プレイヤーの弾をドットマップに描画(縦方向のバイト境界をまたぐ可能性有り)
    pub fn draw(&mut self, dot_map: &mut DotMap) {
        if let Some(cnt) = self.ban_fire_cnt {
            self.ban_fire_cnt = if cnt < 0 {
                if self.explosion_effect_show {
                    // 爆発エフェクトを消す
                    self.erase_shifted(dot_map, self.pos);
                }
                None
            } else {
                if self.explosion_effect_show {
                    // 弾の爆発エフェクト表示
                    self.array_shifted_sprite(dot_map);
                }
                Some(cnt - 1)
            };
        }
        // 弾がなければ何もしない
        if !self.live {
            return;
        }
        // 移動後描画する
        self.array_shifted_sprite(dot_map);
    }
}
impl ArraySprite for Bullet {
    fn pos(&self) -> IVec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        // 描画するのが弾か爆発エフェクトか
        if self.explosion_effect_show {
            &self.explosion_sprite
        } else {
            &self.sprite
        }
    }
}

pub struct Player {
    width: i32,                   // 描画サイズの幅
    canvas_dot_width: i32,        // キャンバスのドット幅
    pub pos: IVec2,               // 左上位置
    pre_pos: IVec2,               // 前回描画時の位置
    const_max_explosion_cnt: i32, // 撃破されてから再出撃までのカウント数(定数)
    explosion_cnt: Option<i32>,   // Some(再出撃までの残りカウント)
    sprite: Vec<u8>,              // 左側から縦8ピクセルずつを8bitのベクタで表す
}
impl Player {
    pub fn new(canvas_dot_width: i32, canvas_dot_height: i32, sprite: Vec<u8>) -> Self {
        Player {
            width: sprite.len() as i32,
            canvas_dot_width,
            pos: IVec2::new(8, canvas_dot_height - 8 * 3),
            pre_pos: IVec2::new(8, canvas_dot_height - 8 * 3),
            const_max_explosion_cnt: 130,
            explosion_cnt: None,
            sprite,
        }
    }
    pub fn update(&mut self) {
        self.pre_pos = self.pos;
        // 撃破後、復活前
        if let Some(cnt) = self.explosion_cnt {
            // 一定時間経過したら復活する
            if cnt > self.const_max_explosion_cnt {
                self.explosion_cnt = None;
                self.pos.x = 8;
                return;
            }
            // カウントを進める
            self.explosion_cnt = Some(cnt + 1);
            return;
        }

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
    pub fn draw(&mut self, dot_map: &mut DotMap) {
        if let Some(_) = self.explosion_cnt {
            return;
        }
        // 前回描画した部分を0で消す
        self.erase(dot_map, self.pre_pos);
        // 移動後描画する
        self.array_sprite(dot_map);
    }
    pub fn remove(&mut self, dot_map: &mut DotMap) {
        self.explosion_cnt = Some(0);
        self.erase(dot_map, self.pos);
    }
}

impl ArraySprite for Player {
    fn pos(&self) -> IVec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        &self.sprite
    }
}
