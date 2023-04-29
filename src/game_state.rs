use std::collections::{HashSet, VecDeque};
use bevy::prelude::*;
use rand::Rng;
use crate::common::*;

pub struct FutureCircle {
    pub kind: i32,
    pub row: usize,
    pub col: usize
}

#[derive(Clone, Copy)]
pub struct CellState(pub i32);

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

    pub fn find_path(&self, from: (usize, usize), to: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut d: Vec<Vec<usize>> = Vec::new();
        let mut p: Vec<Vec<(usize, usize)>> = Vec::new();
        let inf = FIELD_SIZE * FIELD_SIZE;

        for _ in 0..FIELD_SIZE {
            d.push(vec![inf; FIELD_SIZE]);
            p.push(vec![(inf, inf); FIELD_SIZE]);
        }
        d[from.0][from.1] = 0;
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        q.push_back(from);

        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            let nexts = [(cur.0 as i32 - 1, cur.1 as i32), (cur.0 as i32 + 1, cur.1 as i32),
                (cur.0 as i32, cur.1 as i32 - 1), (cur.0 as i32, cur.1 as i32 + 1)];
            for next in nexts {
                if next.0 < 0 || next.0 >= FIELD_SIZE as i32 || next.1 < 0 || next.1 >= FIELD_SIZE as i32 {
                    continue;
                }
                let unext = (next.0 as usize, next.1 as usize);
                if self.cells[unext.0][unext.1].0 != -1 || d[unext.0][unext.1] != inf {
                    continue;
                }
                d[unext.0][unext.1] = d[cur.0][cur.1] + 1;
                p[unext.0][unext.1] = cur;
                q.push_back(unext);
            }
        }

        if d[to.0][to.1] == inf {
            return None;
        }

        let mut path: Vec<(usize, usize)> = Vec::new();
        let mut cur = to;
        while cur != (inf, inf) {
            path.push(cur);
            cur = p[cur.0][cur.1];
        }

        Some(path)
    }

    pub fn process_disappearing_circles(&mut self) -> HashSet<(usize, usize)> {
        let mut result: HashSet<(usize, usize)> = HashSet::new();
        let deltas = [(-1, 0), (0, -1), (1, 1), (-1, 1)];
        for row in 0..FIELD_SIZE {
            for col in 0..FIELD_SIZE {
                for delta in deltas {
                    if let Some(line) = get_line(self, (row, col), delta, 5) {
                        result.extend(line.into_iter());
                    }
                }
            }
        }
        for (row, col) in &result {
            self.cells[*row][*col].0 = -1;
        }
        return result;

        fn get_line(game: &GameState, from: (usize, usize), delta: (i32, i32), len: i32) -> Option<Vec<(usize, usize)>> {
            let to = (from.0 as i32 + delta.0 * len, from.1 as i32 + delta.1 * len);
            if to.0 < 0 || to.0 >= FIELD_SIZE as i32 || to.1 < 0 || to.1 >= FIELD_SIZE as i32 {
                return None;
            }
            let target_kind = game.cells[from.0][from.1].0;
            let mut result: Vec<(usize, usize)> = Vec::with_capacity(len as usize);
            for idx in 0..len {
                result.push(((from.0 as i32 + delta.0 * idx) as usize, (from.1 as i32 + delta.1 * idx) as usize));
                let last = result.last().unwrap();
                if game.cells[last.0][last.1].0 != target_kind {
                    return None;
                }
            }
            Some(result)
        }
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
