#[allow(unused_imports)]
use planner::config::grids::*;
use planner::utils::plot_path;
use planner::{planners, traits::Solver, utils::load_grid};

#[tokio::main]
async fn main() {
    // INFO: Setup
    env_logger::init();
    log::info!("starting up");
    let grid = load_grid(GRID_S).await;
    let start = (0, 0);
    let planner = planners::RayCasting { len: 5, rays: 8 };
    let path = planner.solve(&grid, start);

    match path {
        Some(path) => plot_path(&grid, &path),
        _ => log::debug!("No path found"),
    }
}
