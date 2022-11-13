use std::io::Write;
pub struct DotData {
    pub width: u16,       // 幅[ドット]
    pub height: u16,      // 高さ[ドット]
    pub dot_map: Vec<u8>, // 描画部分は#、非描画部分は_で表す
}

fn convert_dot_map(dot_map: Vec<&str>) -> Vec<u8> {
    let red: Vec<u8> = vec![255, 0, 0, 255];
    let background: Vec<u8> = vec![0, 0, 0, 0];

    let mut bytes: Vec<u8> = Vec::new();
    for line in dot_map {
        let space_removed = line.replace(' ', "");
        for c in space_removed.chars() {
            if c == '#' {
                bytes.write(&red).unwrap();
            } else if c == '_' {
                bytes.write(&background).unwrap();
            }
        }
    }
    bytes
}

pub fn dot_data(name: &str) -> DotData {
    let player = DotData {
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
    let crab_down = DotData {
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
    let crab_banzai = DotData {
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
