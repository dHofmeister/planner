use crate::config::grids::*;
use anyhow::Result;
use std::collections::VecDeque;
use std::fmt;
use std::hash::{Hash, Hasher};

// INFO: Path
#[derive(Clone)]
pub struct Path {
    pub steps: VecDeque<(usize, usize)>,
    pub total_cost: usize,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path: ")?;
        for (x, y) in &self.steps {
            write!(f, "({},{}) ", x, y)?;
        }
        write!(f, "| Cost: {}", self.total_cost)
    }
}

// INFO: Node
#[derive(Clone)]
pub struct Node {
    pub row: usize,
    pub col: usize,
    pub value: usize,
    pub parent: Option<(usize, usize)>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Eq for Node {}

// INFO: Grid
#[derive(Clone)]
pub struct Grid {
    pub data: Vec<u8>,
    pub size: usize,
}

impl Grid {
    pub fn new(data: Vec<u8>) -> Self {
        let size = (data.len() as f64).sqrt() as usize;
        Grid { data, size }
    }

    pub fn value_at(&self, row: usize, col: usize) -> u8 {
        self.data[row * self.size + col]
    }

    pub fn saturated_subtract_at(&mut self, row: usize, col: usize, range: usize, amount: usize) {
        let (start_row, end_row, start_col, end_col) = self.get_start_end_row_col(row, col, range);
        for r in start_row..end_row {
            for c in start_col..end_col {
                let index = r * self.size + c;
                self.data[index] = self.data[index].saturating_sub(amount as u8);
            }
        }
    }

    pub fn saturated_add_at(&mut self, row: usize, col: usize, range: usize, amount: usize) {
        let (start_row, end_row, start_col, end_col) = self.get_start_end_row_col(row, col, range);
        for r in start_row..end_row {
            for c in start_col..end_col {
                let index = r * self.size + c;
                self.data[index] = self.data[index].saturating_add(amount as u8);
            }
        }
    }

    fn get_start_end_row_col(
        &self,
        row: usize,
        col: usize,
        range: usize,
    ) -> (usize, usize, usize, usize) {
        let start_row = row.saturating_sub(range);
        let end_row = (row + range + 1).min(self.size);
        let start_col = col.saturating_sub(range);
        let end_col = (col + range + 1).min(self.size);

        (start_row, end_row, start_col, end_col)
    }

    pub fn load(grid_name: &str) -> Result<Self> {
        let grid_file = match grid_name {
            "GRID_S" => GRID_S,
            "GRID_M" => GRID_M,
            "GRID_L" => GRID_L,
            "GRID_TEST_SINGLE" => GRID_TEST_SINGLE,
            "GRID_TEST_LINE" => GRID_TEST_LINE,
            "GRID_TEST_HORIZONTAL" => GRID_TEST_HORIZONTAL,
            "GRID_TEST_VERTICAL" => GRID_TEST_VERTICAL,
            "GRID_TEST_DIAGONAL" => GRID_TEST_DIAGONAL,
            _ => return Err(anyhow::anyhow!("Invalid grid name: {}", grid_name)),
        };

        let grid_vec: Vec<u8> = grid_file
            .split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok().map(|num| num.saturating_mul(10)))
            .collect();

        let grid = Self::new(grid_vec);

        Ok(grid)
    }

    pub fn max(&mut self, row: usize, col: usize, range: usize, source_grid: &Grid) {
        let (start_row, end_row, start_col, end_col) = self.get_start_end_row_col(row, col, range);
        for r in start_row..end_row {
            for c in start_col..end_col {
                let index = r * self.size + c;
                self.data[index] = self.data[index].max(source_grid.data[index] as u8);
            }
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{} ", self.value_at(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
