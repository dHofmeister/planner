use crate::traits::Planner;
use crate::types::Node;
use crate::types::{Grid, Path};
use priority_queue::PriorityQueue;
use std::collections::HashMap;

pub struct Dijkstra {}

// WARNING: Unfinished
impl Planner for Dijkstra {
    fn solve(&self, grid: &Grid, start: (usize, usize)) -> Option<Path> {
        let mut queue = PriorityQueue::new();
        let node = Node {
            row: start.0,
            col: start.1,
            value: grid.value_at(start.0, start.1) as usize,
            parent: None,
        };

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

        let mut distances = HashMap::new();
        distances.insert((node.row, node.col), 0);

        queue.push(node.clone(), node.value);

        while let Some((current_node, current_value)) = queue.pop() {
            for (dx, dy) in directions {
                let new_row = current_node.row as i32 + *dx;
                let new_col = current_node.col as i32 + *dy;

                if new_row >= 0
                    && new_row < grid.size as i32
                    && new_col >= 0
                    && new_col < grid.size as i32
                {
                    let new_node = Node {
                        row: new_row as usize,
                        col: new_col as usize,
                        value: current_value
                            + grid.value_at(new_row as usize, new_col as usize) as usize,
                        parent: Some((current_node.row, current_node.col)),
                    };
                }

                // TODO: Unfinished
            }
        }
        None
    }
}
