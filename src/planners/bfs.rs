use crate::models::Grid;
use crate::traits::Solver;
use std::collections::VecDeque;

struct BFS {}

impl Solver for BFS {
    async fn solve(grid: &Grid, start: (usize, usize)) -> Vec<u8> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; grid.size * grid.size];

        queue.push_back((start.0, start.1, grid.value_at(start.0, start.1)));
        visited[start.0 * grid.size + start.1] = true;

        let directions: &[(i32, i32)] = &[
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        while let Some((row, col, value)) = queue.pop_front() {
            let current_value: u32 = path.iter().map(|&x| x as u32).sum();
            for (dx, dy) in directions {
                let new_row = row as i32 + dx;
                let new_col = col as i32 + dy;

                if new_row >= 0
                    && new_row < grid.size as i32
                    && new_col >= 0
                    && new_col < grid.size as i32
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    let index = new_row * grid.size + new_col;

                    if !visited[index] {
                        visited[index] = true;
                        let new_value = value + grid.value_at(new_row, new_col);
                        queue.push_back((new_row, new_col, new_value));
                    }
                }
            }
        }
        vec![1]
    }
}
