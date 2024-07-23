use crate::types::{Grid, Path};
use ::std::collections::VecDeque;
use anyhow::Result;
use std::usize;

pub fn plot_paths(grid: &Grid, paths: &Vec<Vec<Path>>) {
    let mut output = String::new();
    output.push('\n');

    for i in 0..grid.size {
        for j in 0..grid.size {
            let value = grid.value_at(i, j);

            let path_info = paths.iter().enumerate().find_map(|(idx, path)| {
                path.last().and_then(|last_path| {
                    let first_step = last_path.steps.front();
                    let any_step = last_path.steps.iter().find(|&step| step == &(i, j));

                    if first_step == Some(&(i, j)) {
                        Some((idx, last_path.total_cost))
                    } else if any_step.is_some() {
                        Some((usize::MAX, last_path.total_cost))
                    } else {
                        None
                    }
                })
            });
            match path_info {
                Some((p_idx, cost)) => {
                    if p_idx != usize::MAX {
                        output.push_str(&format!("{}X{:<3}", p_idx, cost));
                    } else {
                        output.push_str(&format!(" [{:2}]", value));
                    }
                }
                None => output.push_str(&format!("  {:2} ", value)),
            }
        }
        output.push('\n');
    }

    log::info!("Paths on grid:\n{}", output);
}

pub fn create_path_traces(
    paths: &Vec<Vec<Path>>,
    positions: &Vec<(usize, usize)>,
    final_grid: &Result<Grid>,
) -> Vec<Vec<Path>> {
    let mut path_traces: Vec<Vec<Path>> = vec![Vec::new(); positions.len()];

    // INFO: Create initial path traces
    for (index, path_vector) in paths.iter().enumerate() {
        let mut current_drone_path = Path {
            steps: VecDeque::<(usize, usize)>::new(),
            total_cost: 0,
        };
        for path in path_vector.iter() {
            current_drone_path
                .steps
                .push_back(*path.steps.front().unwrap());
        }
        path_traces[index] = vec![current_drone_path];
    }

    // INFO: Calculate total cost for each path trace
    for path_trace in path_traces.iter_mut() {
        let mut total_cost: usize = 0;
        for (x, y) in &path_trace.first().unwrap().steps {
            total_cost += final_grid.as_ref().unwrap().value_at(*x, *y) as usize;
        }
        path_trace.first_mut().unwrap().total_cost = total_cost;
    }

    path_traces
}

pub fn print_paths(paths: &Vec<Vec<Path>>) {
    for (drone_index, drone_paths) in paths.iter().enumerate() {
        println!("Drone {}:", drone_index);
        for (path_index, path) in drone_paths.iter().enumerate() {
            print!("  Path {}: ", path_index);
            for (step_index, &(x, y)) in path.steps.iter().enumerate() {
                if step_index > 0 {
                    print!(", ");
                }
                print!("({},{})", x, y);
            }
            println!(" (Total cost: {})", path.total_cost);
        }
    }
}
