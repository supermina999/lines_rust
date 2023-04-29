pub const FIELD_SIZE: usize = 9;
pub const WINDOW_WIDTH: f32 = 600.;
pub const WINDOW_HEIGHT: f32 = 800.;
pub const CELL_SIZE: f32 = WINDOW_WIDTH / FIELD_SIZE as f32;
pub const CIRCLE_SIZE: f32 = CELL_SIZE * 0.8;

pub const CIRCLE_KINDS: i32 = 7;
pub const CIRCLES_PER_TURN: usize = 3;

pub const CIRCLE_BOUNCE_TIME: f32 = 0.2;
pub const CIRCLE_BOUNCE_SCALE: f32 = 0.3;
pub const CELL_MOVE_TIME: f32 = 0.05;
pub const CIRCLE_DISAPPEAR_TIME: f32 = 0.2;

pub fn get_cell_coords(row: usize, col: usize) -> (f32, f32) {
    let x = -WINDOW_WIDTH / 2. + CELL_SIZE * (col as f32 + 0.5);
    let y = -WINDOW_HEIGHT / 2. + CELL_SIZE * (row as f32 + 0.5);
    (x, y)
}
