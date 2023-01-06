use crate::{array_sprite::ArraySprite, dot_map::DotMap};
use macroquad::prelude::*;
use macroquad::time;

pub struct Explosion {
    pos: IVec2,
    pub show_cnt: Option<i32>, // 生存フラグ(表示残りカウント)
    sprite: Vec<u8>,           // 左側から縦8ピクセルずつを8bitのベクタで表す
}

impl Explosion {
    fn create_effect(&mut self, dot_map: &mut DotMap, pos: IVec2) {
        self.pos = pos;
        self.show_cnt = Some(20);
        // 爆発エフェクトを表示
        self.array_shifted_sprite(dot_map);
    }
    fn update_draw(&mut self, dot_map: &mut DotMap) {
        if let Some(cnt) = self.show_cnt {
            // カウント終了
            if cnt < 0 {
                // 描画した部分を0で消す
                self.erase(dot_map, self.pos);
                self.show_cnt = None;
            } else {
                self.show_cnt = Some(cnt - 1);
            }
        }
    }
}
impl ArraySprite for Explosion {
    fn pos(&self) -> IVec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        &self.sprite
    }
}

pub struct Ufo {
    width: i32,
    canvas_dot_width: i32, // キャンバスのドット幅
    pos: IVec2,            // 左上位置
    pre_pos: IVec2,        // 前回描画時の位置
    live: bool,            // 存在しているか否か
    move_dir: i32,         // 移動方向
    lapse_time: f64,       // 前回画面から消滅したときの時刻
    sprite: Vec<u8>,       // 左側から縦8ピクセルずつを8bitのベクタで表す
    pub explosion: Explosion,
}

impl Ufo {
    pub fn new(canvas_dot_width: i32, sprite: Vec<u8>, explosion_sprite: Vec<u8>) -> Self {
        Ufo {
            width: sprite.len() as i32,
            canvas_dot_width,
            pos: IVec2::new(0, 8),
            pre_pos: IVec2::new(0, 8),
            live: false,
            move_dir: 1,
            lapse_time: time::get_time(),
            sprite,
            explosion: Explosion {
                pos: IVec2::new(0, 0),
                show_cnt: None,
                sprite: explosion_sprite,
            },
        }
    }
    fn remove(&mut self, dot_map: &mut DotMap) {
        self.live = false;
        // 移動方向反転
        self.move_dir *= -1;
        // タイマーリセット
        self.lapse_time = time::get_time();

        // 前回描画した部分を消す
        self.erase(dot_map, self.pre_pos);
    }
    // プレイヤーの弾が当たった場合
    pub fn hit_player_bullet(&mut self, dot_map: &mut DotMap) {
        // UFOを消す
        self.remove(dot_map);
        // 爆発エフェクト描画
        self.explosion.create_effect(dot_map, self.pos);
    }
    pub fn update(&mut self, dot_map: &mut DotMap, fire_cnt: i32) {
        self.pre_pos = self.pos;
        self.explosion.update_draw(dot_map);
        // 画面の反対側まで到達した場合
        if (self.move_dir < 0 && self.pos.x < 8)
            || (0 < self.move_dir && self.canvas_dot_width - 8 <= self.pos.x + self.width)
        {
            self.remove(dot_map);
            return;
        }
        // 移動中
        if self.live {
            self.pos.x += self.move_dir;
        } else {
            // 消滅してから一定時間経過したら
            if time::get_time() - self.lapse_time > 5. {
                // UFOが出現する瞬間
                self.live = true;
                // プレイヤーの発射数が偶数であれば右から左へ動く
                if fire_cnt % 2 == 0 {
                    self.pos.x = self.canvas_dot_width - self.width - 8;
                    self.move_dir = -1;
                } else {
                    // 奇数ならば左から右へ動く
                    self.pos.x = 8;
                    self.move_dir = 1;
                }
            }
        }
        self.draw(dot_map);
    }

    // UFOをドットマップに描画(縦方向のバイト境界はまたがない)
    fn draw(&self, dot_map: &mut DotMap) {
        if !self.live {
            return;
        }
        // 前回描画した部分を0で消す
        self.erase(dot_map, self.pre_pos);
        // 移動後描画する(透過無し)
        self.array_sprite(dot_map);
    }
}

impl ArraySprite for Ufo {
    fn pos(&self) -> IVec2 {
        self.pos
    }
    fn sprite(&self) -> &[u8] {
        &self.sprite
    }
}
