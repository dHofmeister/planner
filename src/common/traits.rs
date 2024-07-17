use anyhow::Result;

use crate::types::{Grid, Path};

pub trait Solver {
    fn solve(&self, grid: &Grid, start: (usize, usize)) -> Option<Path>;
}

pub trait Simulator {
    fn sim(grid: &Grid, path: &Path) -> Result<(Grid, Path)>;
}
