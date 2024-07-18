use crate::traits::Solver;
use crate::types::Node;
use crate::types::{Grid, Path};
use std::collections::{HashMap, VecDeque};

pub struct BFS {}

impl Solver for BFS {
    fn solve(&self, grid: &Grid, start: (usize, usize)) -> Option<Path> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; grid.size * grid.size];
        let mut nodes = HashMap::new();
        let mut max_value = 0;
        let mut max_end = start;

        queue.push_back(start);
        visited[start.0 * grid.size + start.1] = true;
        nodes.insert(
            start,
            Node {
                row: start.0,
                col: start.1,
                value: grid.value_at(start.0, start.1) as u32,
                parent: None,
            },
        );

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

        while let Some((row, col)) = queue.pop_front() {
            let current_value = nodes[&(row, col)].value;

            if current_value > max_value {
                max_value = current_value;
                max_end = (row, col);
            }

            for (dx, dy) in directions {
                let new_row = row as i32 + *dx;
                let new_col = col as i32 + *dy;

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
                        let new_value = current_value + grid.value_at(new_row, new_col) as u32;
                        nodes.insert(
                            (new_row, new_col),
                            Node {
                                row: new_row,
                                col: new_col,
                                value: new_value,
                                parent: Some((row, col)),
                            },
                        );
                        queue.push_back((new_row, new_col));
                    }
                }
            }
        }

        log::debug!(
            "Visited: {}/{}",
            visited.iter().filter(|&&x| x).count(),
            visited.len()
        );

        log::debug!("Nodes in Hashmap: {}", nodes.len());
        log::debug!("Size of grid: {} x {}", grid.size, grid.size);
        log::debug!("Max value found: {}", max_value.clone());

        let mut path = Path {
            steps: VecDeque::new(),
            total_cost: max_value,
        };

        let mut current = max_end;
        while let Some(node) = nodes.get(&current) {
            path.steps.push_back(current);
            if let Some(parent) = node.parent {
                current = parent;
            } else {
                break;
            }
        }

        if path.steps.is_empty() {
            return None;
        }

        path.steps.make_contiguous().reverse();
        log::debug!("Path length: {}", path.steps.len());

        Some(path)
    }
}
