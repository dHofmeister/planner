use crate::traits::Planner;
use crate::types::{Grid, Path};
use bresenham::Bresenham;
use std::f32;
use std::f32::consts;

/// A planner that uses ray casting to find paths.
pub struct RayCasting {
    /// The length of each ray.
    pub len: usize,
    /// The number of rays to cast.
    pub rays: usize,
}

impl Planner for RayCasting {
    /// Solves the path planning problem using ray casting.
    ///
    /// # Arguments
    ///
    /// * `grid` - The grid on which to plan the path.
    /// * `start` - The starting position (x, y) on the grid.
    ///
    /// # Returns
    ///
    /// Returns `Some(Path)` if a path is found, or `None` if no path is possible.
    fn solve(&self, grid: &Grid, start: (usize, usize)) -> Option<Path> {
        let d_angle: f32 = 2.0 * consts::PI / self.rays as f32;
        let mut angles = Vec::<f32>::with_capacity(self.rays);
        let mut goals = Vec::<(isize, isize)>::with_capacity(self.rays);

        let mut max_avg_value: f32 = 0.0;
        let mut best_path: Option<Path> = None;

        // Calculate angles for each ray
        // WARNING: Using rays that are not vertical, horizontal or diagonal could lead do
        // oscillatory behavior
        for i in 0..self.rays {
            angles.push(d_angle * i as f32);
        }

        // Calculate goal positions for each ray
        for angle in angles {
            let dx = f32::cos(angle) * (self.len as f32);
            let dy = f32::sin(angle) * (self.len as f32);

            let gx = f32::round(dx) as isize;
            let gy = f32::round(dy) as isize;

            goals.push((start.0 as isize + gx, start.1 as isize + gy));
        }

        log::debug!("Goals: {:?}", goals);

        // Evaluate each goal
        for goal in goals {
            let mut line_points =
                Bresenham::new((start.0 as isize, start.1 as isize), goal).collect::<Vec<_>>();
            line_points.retain(|&(x, y)| {
                x >= 0 && y >= 0 && x < grid.size as isize && y < grid.size as isize
            });
            let mut line_value: f32 = 0.0;

            // Calculate total value along the line
            for line_point in &line_points {
                line_value += grid.value_at(line_point.0 as usize, line_point.1 as usize) as f32;
            }

            // NOTE: +2 to bias the avg to avoid the map edge
            let line_avg_value = line_value as f32 / (line_points.len() as f32 + 2.);

            let converted_line_points: Vec<(usize, usize)> = line_points
                .into_iter()
                .map(|(x, y)| (x as usize, y as usize))
                .collect();

            // Save best path
            if line_avg_value > max_avg_value && converted_line_points.len() > 1 {
                max_avg_value = line_avg_value;
                best_path = Some(Path {
                    steps: converted_line_points.into(),
                    total_cost: f32::round(line_value) as usize,
                });
            }
        }
        best_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the ray casting planner with a small grid.
    #[test]
    fn test_ray_casting_planner() {
        let planner = RayCasting { len: 3, rays: 8 };

        let grid = Grid::load("GRID_S").expect("Could not load grid");

        let start_pos = (1, 1);
        let path = planner.solve(&grid, start_pos);

        assert!(path.is_some(), "Planner should find a path");

        if let Some(path) = path {
            assert!(!path.steps.is_empty(), "Path should have at least one step");

            assert!(path.steps.len() <= 3, "Path should not exceed ray len");
        }
    }
}
