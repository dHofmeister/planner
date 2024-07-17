use crate::models::Grid;

pub trait Solver {
    fn solve(grid: &Grid, start: (usize, usize)) -> Option<Path>;
}
