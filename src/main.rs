#[allow(unused_imports)]
use planner::config::{GRID_L, GRID_M, GRID_S};
use planner::utils::load_grid;

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("starting up");
    let _grid = load_grid(GRID_S).await;
}
