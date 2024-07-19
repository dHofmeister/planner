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

use log::LevelFilter;
#[tokio::main]
async fn main() {
    // INFO: Initialize
    env_logger::init();
    log::info!("starting up");
    let cli = Cli::parse();

    // INFO: Starting Configuration
    let mut current_location = (cli.pos_x, cli.pos_y);
    let mut grid: Result<Grid> = Grid::load(&cli.grid);

    let planner = planners::RayCasting {
        len: cli.size,
        rays: 8,
    };
    let simulator = simulators::Incremental { step_size: 1 };

    // INFO: Prepare
    let max_steps: usize = cli.time_steps;
    let mut path_trace = Path {
        steps: VecDeque::<(usize, usize)>::new(),
        total_cost: 0,
    };

    // INFO: Run
    let mut current_step = 0;
    let mut deadline = Deadline::new(cli.max_duration as f32);
    while let Ok(current_grid) = grid.as_ref() {
        current_step += 1;
        match planner.solve(&current_grid, current_location) {
            Some(path) => {
                path_trace.steps.push_back(path.steps[0].clone());
                path_trace.total_cost = path.total_cost;

                match simulator.solve(&current_grid, &path) {
                    Ok((new_grid, new_location)) => {
                        grid = Ok(new_grid);
                        current_location = new_location;

                        if log::max_level() >= LevelFilter::Debug {
                            plot_path(&grid.as_ref().unwrap(), &path);
                            thread::sleep(Duration::from_millis(100));
                        }
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

        if current_step == max_steps {
            log::debug!("Max time steps reached");
            break;
        }

        deadline.tick();
        if deadline.will_exceed_deadline() {
            log::debug!("Terminating due to deadline");
            break;
        }
    }

    // INFO: End
    let final_grid = Grid::load(&cli.grid);
    if !path_trace.steps.is_empty() && final_grid.is_ok() {
        plot_path(&final_grid.unwrap(), &path_trace);
    } else {
        log::debug!("No path found");
    }
}
