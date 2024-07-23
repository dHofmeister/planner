use crate::types::{Grid, Path};
use anyhow::Result;

/// A trait for path planning algorithms.
///
/// Implementors of this trait should provide a method to solve
/// path planning problems on a given grid.
pub trait Planner {
    /// Attempts to find a path on the given grid from the start position.
    ///
    /// # Arguments
    ///
    /// * `grid` - The grid on which to plan the path.
    /// * `start` - The starting position (x, y) on the grid.
    ///
    /// # Returns
    ///
    /// Returns `Some(Path)` if a path is found, or `None` if no path is possible.
    fn solve(&self, grid: &Grid, start: (usize, usize)) -> Option<Path>;
}

/// A trait for simulation algorithms.
///
/// Implementors of this trait should provide a method to simulate
/// the execution of a path on a given grid.
pub trait Simulator {
    /// Simulates the execution of a path on the given grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - The initial state of the grid.
    /// * `path` - The path to simulate.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - On success: A tuple with the updated grid and the final position (x, y).
    /// - On failure: An error describing what went wrong.
    fn solve(&self, grid: &Grid, path: &Path) -> Result<(Grid, (usize, usize))>;
}
