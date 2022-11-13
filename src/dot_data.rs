use std::io::Write;
pub struct DotShape {
    pub width: u16,              // 幅[ドット]
    pub height: u16,             // 高さ[ドット]
    pub dot_map: Vec<Vec<bool>>, // 描画部分は#、非描画部分は_で表す
}

impl DotShape {
    // 真偽値で表されたドットマップをmacroquadで利用できるVec<u8>に変換
    pub fn create_color_dot_map(&self, color_str: &str) -> Vec<u8> {
        // 指定されたサイズと実際のドットマップのサイズが一致しているか確認
        if self.height as usize != self.dot_map.len() {
            panic!("指定されたドットマップの高さが実際のデータと異なります。");
        }
        if self.width as usize != self.dot_map[0].len() {
            panic!("指定されたドットマップの幅が実際のデータと異なります。");
        }
        // ドットマップの幅が異なる行が無いか確認
        let map_width = self.dot_map[0].len();
        for l in &self.dot_map {
            if l.len() != map_width {
                panic!("ドットマップの形が不正です。");
            }
        }

        let color = set_color(color_str);
        let background: Vec<u8> = vec![0, 0, 0, 0];

        let mut bytes: Vec<u8> = Vec::new();
        for line in &self.dot_map {
            for c in line {
                if *c {
                    bytes.write(&color).unwrap();
                } else {
                    bytes.write(&background).unwrap();
                }
            }
        }
        bytes
    }
}

// ドットデータを変更する際はこの中身のみ変更する
pub fn dot_data(name: &str) -> DotShape {
    let player = DotShape {
        width: 13,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ # _ _ _ _ _ _",
            "_ _ _ _ _ # # # _ _ _ _ _",
            "_ _ _ _ _ # # # _ _ _ _ _",
            "_ # # # # # # # # # # # _",
            "# # # # # # # # # # # # #",
            "# # # # # # # # # # # # #",
            "# # # # # # # # # # # # #",
            "# # # # # # # # # # # # #",
        ]),
    };
    let crab_down = DotShape {
        width: 11,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ # _ _ _ _ _ # _ _",
            "_ _ _ # _ _ _ # _ _ _",
            "_ _ # # # # # # # _ _",
            "_ # # _ # # # _ # # _",
            "# # # # # # # # # # #",
            "# _ # # # # # # # _ #",
            "# _ # _ _ _ _ _ # _ #",
            "_ _ _ # # _ # # _ _ _",
        ]),
    };
    let crab_banzai = DotShape {
        width: 11,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ # _ _ _ _ _ # _ _",
            "# _ _ # _ _ _ # _ _ #",
            "# _ # # # # # # # _ #",
            "# # # _ # # # _ # # #",
            "# # # # # # # # # # #",
            "_ # # # # # # # # # _",
            "_ _ # _ _ _ _ _ # _ _",
            "_ # _ _ _ _ _ _ _ # _",
        ]),
    };

    match name {
        "player" => player,
        "crab_down" => crab_down,
        "crab_banzai" => crab_banzai,
        _ => panic!(
            "{}のドットマップ取得に失敗しました。プログラムを終了します。",
            name
        ), // ドットマップ取得失敗
    }
}
// 描画部分を真、非描画部分を偽とするドットマップを返す
fn convert_dot_map(dot_map: Vec<&str>) -> Vec<Vec<bool>> {
    let mut bool_map = Vec::new();
    for line in dot_map {
        let mut bool_line = Vec::new();
        // 空白をすべて削除
        let space_removed = line.replace(' ', "");
        for c in space_removed.chars() {
            if c == '#' {
                bool_line.push(true);
            } else if c == '_' {
                bool_line.push(false);
            }
        }
        bool_map.push(bool_line);
    }
    bool_map
}

// 指定した色に対応するrgbaの値を返す
fn set_color(color: &str) -> Vec<u8> {
    match color {
        "TURQUOISE" => vec![68, 200, 210, 255], // 青緑色
        "PURPLE" => vec![219, 85, 221, 255],    // 紫色
        _ => panic!("{}色には対応していません。プログラムを終了します。", color),
    }
}
