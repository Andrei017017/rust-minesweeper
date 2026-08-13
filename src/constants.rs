pub const CELL_SIZE: f32 = 20.;
pub const OFFSET_X: f32 = 10.;
pub const OFFSET_Y: f32 = 100.;
pub const THICKNESS: f32 = 2.;
pub const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];
pub const FONT_BYTES: &[u8] = include_bytes!("DigitTech7-Regular.ttf") ;