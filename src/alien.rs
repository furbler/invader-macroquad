use crate::dot_map::DotMap;
use macroquad::prelude::*;
struct Alien {
    width: i32,
    pos: IVec2,
    sprite1: Vec<u8>,
    sprite2: Vec<u8>,
    show_sprite: bool, // 真のときsprite1、偽のときsprite2とする
}

impl Alien {
    // エイリアンをドットマップに描画(縦方向のバイト境界はまたがない)
    fn array_sprite(&mut self, dot_map: &mut DotMap) {
        // 前回描画した部分を0で消す
        let char_y = (self.pos.y / 8) as usize;
        for dx in 0..self.width as usize {
            dot_map.map[char_y][self.pos.x as usize + dx] = 0;
        }
        // 移動後描画する
        let char_y = (self.pos.y / 8) as usize;
        for dx in 0..self.width as usize {
            dot_map.map[char_y][self.pos.x as usize + dx] = self.sprite1[dx];
        }
    }
}

pub struct AlienManage {
    // エイリアンすべて
    aliens_list: Vec<Alien>,
}

impl AlienManage {
    pub fn new(
        // 下2列のエイリアンのスプライト
        low_sprite1: Vec<u8>,
        low_sprite2: Vec<u8>,
        // 中2列のエイリアンのスプライト
        middle_sprite1: Vec<u8>,
        middle_sprite2: Vec<u8>,
        // 上1列のエイリアンのスプライト
        high_sprite1: Vec<u8>,
        high_sprite2: Vec<u8>,
    ) -> Self {
        let mut aliens_list = Vec::new();
        for _ in 0..22 {
            aliens_list.push(Alien {
                width: low_sprite1.len() as i32,
                pos: IVec2::new(0, 0),
                sprite1: low_sprite1.clone(),
                sprite2: low_sprite2.clone(),
                show_sprite: true,
            })
        }
        for _ in 0..22 {
            aliens_list.push(Alien {
                width: middle_sprite1.len() as i32,
                pos: IVec2::new(0, 0),
                sprite1: middle_sprite1.clone(),
                sprite2: middle_sprite2.clone(),
                show_sprite: true,
            })
        }

        for _ in 0..11 {
            aliens_list.push(Alien {
                width: high_sprite1.len() as i32,
                pos: IVec2::new(0, 0),
                sprite1: high_sprite1.clone(),
                sprite2: high_sprite2.clone(),
                show_sprite: true,
            })
        }
        AlienManage { aliens_list }
    }
    // エイリアンを初期位置に配置
    pub fn init_all_aliens(&mut self) {
        let ref_pos_x = 24;
        let mut ref_pos_y = 12 * 8;
        for y in 0..5 {
            for x in 0..11 {
                self.aliens_list[y * 11 + x].pos = IVec2::new(ref_pos_x + x as i32 * 16, ref_pos_y);
            }
            ref_pos_y -= 16;
        }
    }
    pub fn array_sprite(&mut self, dot_map: &mut DotMap) {
        self.aliens_list.iter_mut().for_each(|alien| {
            alien.array_sprite(dot_map);
        });
    }
}
