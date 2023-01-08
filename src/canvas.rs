// 1文字8ピクセル分がいくつ入るか
pub const CHAR_WIDTH: i32 = 28;
pub const CHAR_HEIGHT: i32 = 26;
// ドット単位の大きさ
pub const DOT_WIDTH: i32 = 8 * CHAR_WIDTH;
pub const DOT_HEIGHT: i32 = 8 * CHAR_HEIGHT;
// 最終的に表示されるディスプレイの大きさ
// 幅は変わらない
pub const ALL_DOT_WIDTH: i32 = DOT_WIDTH;
// 上のスコア表示用の4文字分 + 下の残機表示用の1文字分を加える
pub const ALL_DOT_HEIGHT: i32 = DOT_HEIGHT + 8 * 5;
// 1ドットを何ピクセル四方で表示するか(pixel / dot)
pub const SCALE: i32 = 3;
