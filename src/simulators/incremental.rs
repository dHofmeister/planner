pub use crate::traits::Simulator;
pub use crate::types::{Grid, Path};
use anyhow::Result;

pub struct Incremental {
    pub increment_step: u8,
    pub start_grid: Grid,
}

impl Simulator for Incremental {
    fn solve(&self, grid: &Grid, path: &Path) -> Result<(Grid, (usize, usize))> {
        let (x, y) = path.steps[0];
        let mut out_grid = grid.clone();
        out_grid.data[x * grid.size + y] = 0;
        self.recover(&mut out_grid.data);

        Ok((out_grid, path.steps[1]))
    }
}

impl Incremental {
    // INFO: Incrementally recovers the grid to its original values
    fn recover(&self, b: &mut Vec<u8>) {
        b.iter_mut()
            .zip(self.start_grid.data.iter())
            .for_each(|(b_elem, &a_elem)| {
                if a_elem > *b_elem {
                    *b_elem = b_elem.saturating_add(self.increment_step);
                }
            });
    }
}
