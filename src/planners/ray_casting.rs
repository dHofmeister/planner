use crate::traits::Solver;
use crate::types::{Grid, Path};
use bresenham::Bresenham;
use std::collections::VecDeque;
use std::f32;
use std::f32::consts;

pub struct RayCasting {
    pub len: i32,
    pub rays: i32,
}

impl Solver for RayCasting {
    fn solve(&self, grid: &Grid, start: (usize, usize)) -> Option<Path> {
        let d_angle = 2.0 * consts::PI / self.rays as f32;
        let mut angles = Vec::with_capacity(self.rays as usize);

        let mut goals = VecDeque::<(isize, isize)>::new();
        let mut max_avg_value: f32 = 0.0;
        let mut best_path: Option<Path> = None;

        for i in 0..self.rays {
            angles.push(d_angle * i as f32);
        }

        for angle in angles {
            let dx = f32::cos(angle) * (self.len as f32);
            let dy = f32::sin(angle) * (self.len as f32);

            let gx = f32::round(dx) as isize;
            let gy = f32::round(dy) as isize;

            goals.push_back((gx, gy));
        }

        for goal in goals {
            let mut line_points =
                Bresenham::new((start.0 as isize, start.1 as isize), goal).collect::<Vec<_>>();
            line_points.retain(|&(x, y)| {
                x >= 0 && y >= 0 && x < grid.size as isize && y < grid.size as isize
            });
            let mut line_value: f32 = 0.0;

            for line_point in &line_points {
                line_value += grid.value_at(line_point.0 as usize, line_point.1 as usize) as f32;
            }

            let line_avg_value = line_value as f32 / (line_points.len() as f32 + 2.);

            let converted_line_points: Vec<(usize, usize)> = line_points
                .into_iter()
                .map(|(x, y)| (x as usize, y as usize))
                .collect();
            if line_avg_value > max_avg_value {
                max_avg_value = line_avg_value;
                best_path = Some(Path {
                    steps: converted_line_points.into(),
                    total_cost: f32::round(line_value) as u32,
                });
            }
        }
        best_path
    }
}
