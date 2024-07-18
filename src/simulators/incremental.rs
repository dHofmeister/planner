pub use crate::traits::Simulator;
pub use crate::types::{Grid, Path};
use anyhow::Result;

pub struct Incremental {
    pub step_size: u8,
}

impl Simulator for Incremental {
    fn solve(&self, grid: &Grid, path: &Path) -> Result<(Grid, (usize, usize))> {
        let (x, y) = path.steps[0];
        let mut out_grid = grid.clone();
        out_grid.data[x * grid.size + y] = grid.value_at(x, y).saturating_sub(self.step_size);

        Ok((out_grid, path.steps[1]))
    }
}
