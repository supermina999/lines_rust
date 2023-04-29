pub const FIELD_SIZE: usize = 9;
pub const WINDOW_WIDTH: f32 = 600.;
pub const WINDOW_HEIGHT: f32 = 800.;

pub const CIRCLE_KINDS: i32 = 7;
pub const CIRCLES_PER_TURN: usize = 3;
pub const CELL_MOVE_TIME: f32 = 0.05;

pub fn get_cell_size() -> f32 {
    WINDOW_WIDTH / FIELD_SIZE as f32
}

pub fn get_cell_coords(row: usize, col: usize) -> (f32, f32) {
    let cell_size = WINDOW_WIDTH / FIELD_SIZE as f32;
    let x = -WINDOW_WIDTH / 2. + cell_size * (col as f32 + 0.5);
    let y = -WINDOW_HEIGHT / 2. + cell_size * (row as f32 + 0.5);
    (x, y)
}
