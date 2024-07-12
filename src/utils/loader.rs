use crate::models;

pub async fn load_grid(grid_str: &str) -> models::Grid {
    let grid_vec: Vec<u8> = grid_str
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let grid = models::Grid::new(grid_vec);

    log::debug!("{}", grid);

    grid
}
