#[allow(unused_imports)]
use planner::config::grids::*;
use planner::planners;
use planner::simulators;
use planner::traits::{Simulator, Solver};
use planner::types::{Grid, Path};
use planner::utils::{load_grid, plot_path};

use anyhow::Result;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // INFO: Setup
    env_logger::init();
    log::info!("starting up");
    let mut current_location = (0, 0);

    let mut grid: Result<Grid> = load_grid(GRID_S).await;
    let mut path: Option<Path> = None;

    let planner = planners::RayCasting { len: 7, rays: 8 };
    let simulator = simulators::Incremental { step_size: 1 };

    // INFO: Run
    while let Ok(current_grid) = grid.as_ref() {
        path = planner.solve(&current_grid, current_location);

        if let Some(current_path) = path.as_ref() {
            match simulator.solve(&current_grid, &current_path) {
                Ok((new_grid, new_location)) => {
                    grid = Ok(new_grid);
                    current_location = new_location;

                    thread::sleep(Duration::from_millis(100));

                    plot_path(&grid.as_ref().unwrap(), current_path);
                }
                Err(e) => {
                    log::error!("Simulation error: {:?}", e);
                    break;
                }
            }
        } else {
            break;
        }
    }

    // INFO: End
    let final_grid = grid.unwrap();
    match path {
        Some(path) => plot_path(&final_grid, &path),
        _ => log::debug!("No path found"),
    }
}
