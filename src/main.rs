use planner::parsers::Cli;
use planner::planners;
use planner::simulators;
use planner::traits::{Simulator, Solver};
use planner::types::{Grid, Path};
use planner::utils::{plot_path, Deadline};

#[allow(unused_imports)]
use planner::config::grids::*;

use anyhow::Result;
use clap::Parser;
use std::collections::VecDeque;
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
    env_logger::init();
    log::info!("starting up");
    let cli = Cli::parse();

    // INFO: Starting Configuration
    let mut positions: Vec<(usize, usize)> = cli.pos_x.into_iter().zip(cli.pos_y).collect();
    let mut grid: Result<Grid> = Grid::load(&cli.grid);
    let mut paths = Vec::<Path>::new();

    let planner = planners::RayCasting {
        len: cli.size,
        rays: 8,
    };
    let simulator = simulators::Incremental {
        start_grid: grid
            .as_ref()
            .map(|g| g.clone())
            .expect("Failed to load grid into simulator"),
        increment_step: 0,
    };

    // INFO: Prepare
    let max_steps: usize = cli.time_steps;
    let mut path_trace = Path {
        steps: VecDeque::<(usize, usize)>::new(),
        total_cost: 0,
    };

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

        let reference_grid = global_grid.clone();
        for pos in &positions {
            // INFO: Reduce the value of the grid nearby drones
            global_grid.saturated_subtract_at(pos.0, pos.1, 1, 3);
        }

        log::debug!("Grid: \n {:?}", global_grid);
        log::debug!("Positions: {:?}", positions);

        // INFO: Define per-drone configuration
        for private_location in &mut positions {
            let mut private_grid = global_grid.clone();
            // INFO: Restore the value of the drone currently being processed
            // Other drones' locations remain at reduced value
            private_grid.max(private_location.0, private_location.1, 1, &reference_grid);

            // INFO: Solve Action
            match planner.solve(&private_grid, *private_location) {
                Some(path) => {
                    paths.push(path.clone());
                    path_trace.steps.push_back(path.steps[0].clone());
                    path_trace.total_cost +=
                        global_grid.value_at(path.steps[0].0, path.steps[0].1) as usize;

                    // INFO: Solve Result
                    match simulator.solve(&private_grid, &path) {
                        Ok((new_grid, new_location)) => {
                            grid = Ok(new_grid);
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
            let end_path = paths.last().unwrap().clone();
            plot_path(&grid.as_ref().unwrap(), &end_path);
            thread::sleep(Duration::from_millis(100));
        }
    }

    // INFO: End
    // let final_grid = Grid::load(&cli.grid);
    // if !path_trace.steps.is_empty() && final_grid.is_ok() {
    //     plot_paths(&final_grid.unwrap(), &path_trace);
    // } else {
    //     log::debug!("No path found");
    // }
}
