use std::collections::HashSet;
use bevy::prelude::*;
use rand::Rng;
use crate::constants::*;

pub struct FutureCircle {
    pub kind: i32,
    pub row: usize,
    pub col: usize
}

pub struct CellState(pub(crate) i32);

impl Default for CellState {
    fn default() -> Self {
        CellState(-1)
    }
}

#[derive(Resource)]
pub struct GameState {
    pub cells: Vec<Vec<CellState>>,
    pub future_circles: Vec<FutureCircle>
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            cells: {
                let mut cells: Vec<Vec<CellState>> = Vec::new();
                for _ in 0..FIELD_SIZE {
                    let mut row_cells: Vec<CellState> = Vec::new();
                    for _ in 0..FIELD_SIZE {
                        row_cells.push(CellState::default());
                    }
                    cells.push(row_cells);
                }
                cells
            },
            future_circles: Vec::new()
        }
    }
}

impl GameState {
    pub fn get_free_count(&self) -> usize {
        let mut result = 0;
        for row in &self.cells {
            for cell in row {
                if cell.0 == -1 {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn get_nth_free(&self, idx: usize) -> (usize, usize) {
        let mut cur_free = 0;
        for (row_idx, row) in self.cells.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell.0 == -1 {
                    if cur_free == idx {
                        return (row_idx, col_idx);
                    }
                    cur_free += 1;
                }
            }
        }
        panic!("Can't find Nth free cell");
    }

    pub fn random_future_circles(&mut self, count: usize) -> Result<(), ()> {
        self.future_circles.clear();
        let free_cnt = self.get_free_count();
        if free_cnt < count {
            return Err(());
        }

        let mut rng = rand::thread_rng();
        let mut idx_set: HashSet<usize> = HashSet::new();
        for _ in 0..count {
            loop {
                let idx = rng.gen_range(0..free_cnt);
                if !idx_set.contains(&idx) {
                    idx_set.insert(idx);
                    break;
                }
            }
        }

        for idx in idx_set {
            let (row, col) = self.get_nth_free(idx);
            self.future_circles.push(FutureCircle {
                kind: rng.gen_range(0..CIRCLE_KINDS),
                row,
                col
            })
        }

        return Ok(());
    }
}
