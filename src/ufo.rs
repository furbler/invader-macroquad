use crate::dot_map::DotMap;
use macroquad::prelude::*;
use macroquad::time;

pub struct Ufo {
    width: i32,
    canvas_dot_width: i32, // キャンバスのドット幅
    pos: IVec2,            // 左上位置
    pre_pos: IVec2,        // 前回描画時の位置
    live: bool,            // 弾が存在しているか否か
    move_dir: i32,         // 移動方向
    lapse_time: f64,       // 前回画面から消滅したときの時刻
    sprite: Vec<u8>,       // 左側から縦8ピクセルずつを8bitのベクタで表す
}

impl Ufo {
    pub fn new(canvas_dot_width: i32, sprite: Vec<u8>) -> Self {
        Ufo {
            width: sprite.len() as i32,
            canvas_dot_width,
            pos: IVec2::new(0, 8),
            pre_pos: IVec2::new(0, 8),
            live: false,
            move_dir: 1,
            lapse_time: time::get_time(),
            sprite,
        }
    }
    fn erase(&mut self, dot_map: &mut DotMap) {
        self.live = false;
        // 移動方向反転
        self.move_dir *= -1;
        // タイマーリセット
        self.lapse_time = time::get_time();

        // 前回描画した部分を0で消す
        let char_y = (self.pre_pos.y / 8) as usize;
        for dx in 0..self.width as usize {
            dot_map.map[char_y][self.pre_pos.x as usize + dx] = 0;
        }
    }
    pub fn update(&mut self, dot_map: &mut DotMap) {
        // 画面の反対側まで到達した場合
        if (self.move_dir < 0 && self.pos.x < 8)
            || (0 < self.move_dir && self.canvas_dot_width - 8 <= self.pos.x + self.width)
        {
            self.erase(dot_map);
            return;
        }
        // 出現状態
        if self.live {
            self.pos.x += self.move_dir;
        } else {
            // 消滅してから一定時間経過したら
            if time::get_time() - self.lapse_time > 5. {
                // 出現
                self.live = true;
                if self.move_dir < 0 {
                    self.pos.x = self.canvas_dot_width - self.width - 8;
                } else {
                    self.pos.x = 8;
                }
            }
        }
    }

    // プレイヤーをドットマップに描画(縦方向のバイト境界はまたがない)
    pub fn array_sprite(&mut self, dot_map: &mut DotMap) {
        if !self.live {
            return;
        }
        // 前回描画した部分を0で消す
        let char_y = (self.pre_pos.y / 8) as usize;
        for dx in 0..self.width as usize {
            dot_map.map[char_y][self.pre_pos.x as usize + dx] = 0;
        }
        // 移動後描画する(透過無し)
        let char_y = (self.pos.y / 8) as usize;
        for dx in 0..self.width as usize {
            dot_map.map[char_y][self.pos.x as usize + dx] = self.sprite[dx];
        }
        self.pre_pos = self.pos;
    }
}
