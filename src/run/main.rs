use crate::parsers::Cli;
use crate::traits::{Planner, Simulator};
use crate::types::{Grid, Path};
use crate::utils::{plot_paths, Deadline};

#[allow(unused_imports)]
use crate::config::grids::*;

use anyhow::Result;
use log::LevelFilter;
use std::thread;
use std::time::Duration;

/// Runs a drone simulation based on the provided parameters.
///
/// # Arguments
///
/// * `cli` - Command line interface arguments
/// * `planner` - An implementation of the Planner trait
/// * `simulator` - An implementation of the Simulator trait
/// * `starting_positions` - Initial positions of the drones
/// * `starting_grid` - The initial state of the grid
///
/// # Returns
///
/// A Result containing a vector of paths for each drone, or an error
pub fn run_drone_simulation(
    cli: Cli,
    planner: impl Planner,
    simulator: impl Simulator,
    starting_positions: Vec<(usize, usize)>,
    starting_grid: &Grid,
) -> Result<Vec<Vec<Path>>> {
    // Prepare simulation
    let mut positions = starting_positions.clone();
    let mut paths: Vec<Vec<Path>> = vec![Vec::new(); positions.len()];
    let mut grid: Result<Grid> = Ok(starting_grid.clone());
    let max_steps: usize = cli.time_steps;

    // Run simulation
    let mut current_step = 0;
    let mut deadline = Deadline::new(cli.max_duration as f32);

    loop {
        current_step += 1;

        // Prepare Grid
        let mut global_grid = match grid.as_mut() {
            Ok(g) => g.clone(),
            Err(_) => return Ok(paths),
        };

        let reference_grid = global_grid.clone();
        for pos in &positions {
            global_grid.saturated_subtract_at(pos.0, pos.1, 2, 5);
        }

        log::debug!("Grid: \n {:?}", global_grid);
        log::debug!("Positions: {:?}", positions);

        // Define per-drone configuration
        let mut private_grid = global_grid.clone();
        let n_pos = positions.len();
        for (index, private_location) in positions.iter_mut().enumerate() {
            private_grid.max(private_location.0, private_location.1, 1, &reference_grid);

            // Plan Action
            // WARNING: Current algorithm is not fully resistant to drone convergence and
            // collapse
            match planner.solve(&private_grid, *private_location) {
                Some(path) => {
                    paths[index].push(path.clone());

                    // Simulate Result
                    // BUG: Simulator will increment n_drones times in 1 iteration
                    match simulator.solve(&private_grid, &path) {
                        Ok((new_grid, new_location)) => {
                            if index != n_pos - 1 {
                                private_grid = new_grid;
                            } else {
                                grid = Ok(new_grid);
                            }
                            *private_location = new_location;
                        }
                        Err(e) => {
                            log::error!("Simulation error: {:?}", e);
                            break;
                        }
                    }
                }
                _ => {
                    log::debug!("Path planning finished");
                    break;
                }
            }
        }

        if log::max_level() >= LevelFilter::Debug {
            plot_paths(&reference_grid, &paths);
            thread::sleep(Duration::from_millis(100));
        }

        if current_step == max_steps {
            log::info!("Max time steps reached");
            return Ok(paths);
        }

        deadline.tick();
        if deadline.will_exceed_deadline() {
            log::info!("Terminating due to deadline");
            return Ok(paths);
        }
    }
}
