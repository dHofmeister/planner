use crate::types::{Grid, Path};

pub fn plot_path(grid: &Grid, path: &Path) {
    let mut output = String::new();
    output.push('\n');

    for i in 0..grid.size {
        for j in 0..grid.size {
            let value = grid.value_at(i, j);
            if !path.steps.is_empty() && path.steps.front() == Some(&(i, j)) {
                output.push_str(&format!("[ X]"));
            } else if path.steps.contains(&(i, j)) {
                output.push_str(&format!("[{:2}]", value));
            } else {
                output.push_str(&format!(" {:2} ", value));
            }
        }
        output.push('\n');
    }
    log::debug!("Path on grid: {}", output);
}
