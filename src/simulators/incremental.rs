pub use crate::traits::Simulator;
pub use crate::types::{Grid, Path};
use anyhow::Result;

struct IncrementalSim {}

// impl Simulator for IncrementalSim {
//     fn sim(grid: &Grid, path: &Path) -> Result<(Grid, Path)> {}
// }
