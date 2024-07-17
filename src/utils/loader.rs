use crate::types::Grid;
pub async fn load_grid(grid_str: &str) -> Grid {
    let grid_vec: Vec<u8> = grid_str
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let grid = Grid::new(grid_vec);

    log::debug!("{}", grid);

    grid
}
