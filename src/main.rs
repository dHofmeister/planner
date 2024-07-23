use planner::parsers::Cli;
use planner::planners;
use planner::run;
use planner::simulators;
use planner::types::Grid;
use planner::utils::{create_path_traces, plot_paths, print_paths};

#[allow(unused_imports)]
use planner::config::grids::*;

use clap::Parser;

// TODO: List of todos:
// --docs
// --bench

#[tokio::main]
async fn main() {
    // INFO: Initialize
    // NOTE: Run in RUST_LOG=DEBUG for visualisations, RUST_LOG=INFO for results-only output
    env_logger::init();
    log::info!("starting up");
    let cli = Cli::parse();

    // INFO: Starting Configuration
    let x = cli.pos_x.clone();
    let y = cli.pos_y.clone();
    let positions: Vec<(usize, usize)> = x.into_iter().zip(y).collect();
    let grid: Grid = Grid::load(&cli.grid).expect("Failed to load the grid");

    let planner = planners::RayCasting {
        len: cli.size,
        rays: 16,
    };
    let simulator = simulators::Incremental {
        start_grid: grid.clone(),
        increment_step: 1,
    };
    let paths = match run::run_drone_simulation(cli, planner, simulator, positions.clone(), &grid) {
        Ok(result_path) => result_path,
        Err(_) => {
            log::error!("Failed the simulation");
            return;
        }
    };

    // INFO: End
    // Aggregate the results and format them for printing
    let path_traces = create_path_traces(&paths, &positions, &grid);

    if !path_traces.is_empty() {
        plot_paths(&grid, &path_traces);
        print_paths(&path_traces);
    } else {
        log::debug!("No path found");
    }
}
