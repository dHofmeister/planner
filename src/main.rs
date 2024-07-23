use planner::parsers::Cli;
use planner::planners;
use planner::simulators;
use planner::traits::{Simulator, Solver};
use planner::types::{Grid, Path};
use planner::utils::{create_path_traces, plot_paths, print_paths, Deadline};

#[allow(unused_imports)]
use planner::config::grids::*;

use anyhow::Result;
use clap::Parser;
use std::thread;
use std::time::Duration;

// TODO: List of todos:
// --swarm
// --example
// --docs
// --bench
// --test

use log::LevelFilter;
#[tokio::main]
async fn main() {
    // INFO: Initialize

    // NOTE: Run in RUST_LOG=DEBUG for visualisations, RUST_LOG=INFO for results-only output
    env_logger::init();
    log::info!("starting up");
    let cli = Cli::parse();

    // INFO: Starting Configuration
    let mut positions: Vec<(usize, usize)> = cli.pos_x.into_iter().zip(cli.pos_y).collect();
    let mut grid: Result<Grid> = Grid::load(&cli.grid);

    let planner = planners::RayCasting {
        len: cli.size,
        rays: 16,
    };
    let simulator = simulators::Incremental {
        start_grid: grid
            .as_ref()
            .map(|g| g.clone())
            .expect("Failed to load grid into simulator"),
        increment_step: 1,
    };

    // INFO: Prepare
    let mut paths: Vec<Vec<Path>> = vec![Vec::new(); positions.len()];
    let max_steps: usize = cli.time_steps;

    // INFO: Run
    let mut current_step = 0;
    let mut deadline = Deadline::new(cli.max_duration as f32);

    loop {
        current_step += 1;

        // INFO: Prepare Grid
        let mut global_grid = match grid.as_mut() {
            Ok(g) => g.clone(),
            Err(_) => break,
        };

        // TODO: Reduce clones, better track grid preperation
        let reference_grid = global_grid.clone();
        for pos in &positions {
            // INFO: Reduce the value of the grid nearby drones
            global_grid.saturated_subtract_at(pos.0, pos.1, 1, 3);
        }

        log::debug!("Grid: \n {:?}", global_grid);
        log::debug!("Positions: {:?}", positions);

        // INFO: Define per-drone configuration
        let mut private_grid = global_grid.clone();
        let n_pos = positions.len();
        for (index, private_location) in positions.iter_mut().enumerate() {
            // INFO: Restore the value of the drone currently being processed
            // Other drones' locations remain at reduced value
            private_grid.max(private_location.0, private_location.1, 1, &reference_grid);

            // INFO: Plan Action
            // WARNING: Current planners are not immune to multi drone convergence and stacking
            match planner.solve(&private_grid, *private_location) {
                Some(path) => {
                    paths[index].push(path.clone());

                    // INFO: Simulate Result
                    // FIX: Current simulator increments for every drone, not for every loop
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

        if current_step == max_steps {
            log::info!("Max time steps reached");
            break;
        }

        deadline.tick();
        if deadline.will_exceed_deadline() {
            log::info!("Terminating due to deadline");
            break;
        }

        if log::max_level() >= LevelFilter::Debug {
            plot_paths(&reference_grid, &paths);
            thread::sleep(Duration::from_millis(100));
        }
    }

    // INFO: End
    // Aggregate the results and format them for printing
    let final_grid = Grid::load(&cli.grid);
    let path_traces = create_path_traces(&paths, &positions, &final_grid);

    if !path_traces.is_empty() && final_grid.is_ok() {
        plot_paths(&final_grid.unwrap(), &path_traces);
        print_paths(&path_traces);
    } else {
        log::debug!("No path found");
    }
}
