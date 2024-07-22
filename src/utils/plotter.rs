use crate::types::{Grid, Path};

pub fn plot_paths(grid: &Grid, paths: &Vec<Vec<Path>>) {
    let mut output = String::new();
    output.push('\n');

    for i in 0..grid.size {
        for j in 0..grid.size {
            let value = grid.value_at(i, j);

            let path_info = paths.iter().enumerate().find_map(|(idx, path)| {
                path.last().and_then(|first_path| {
                    first_path.steps.front().and_then(|first_step| {
                        if first_step == &(i, j) {
                            Some((idx, 0))
                        } else {
                            None
                        }
                    })
                })
            });

            match path_info {
                Some((p_idx, _)) => output.push_str(&format!("[X{}]", p_idx)),
                None => output.push_str(&format!(" {:2} ", value)),
            }
        }
        output.push('\n');
    }

    // output.push_str("Captured Points:\n");
    // for (idx, path) in paths.iter().enumerate() {
    //     output.push_str(&format!("Score: {}\n", path.total_cost));
    // }

    log::info!("Paths on grid:\n{}", output);
}

pub fn plot_path(grid: &Grid, path: &Path) {
    let mut output = String::new();
    output.push('\n');

    for i in 0..grid.size {
        for j in 0..grid.size {
            let value = grid.value_at(i, j);
            if !path.steps.is_empty() && path.steps.front() == Some(&(i, j)) {
                output.push_str(&format!("[ X]"));
            } else if let Some(index) = path.steps.iter().position(|&step| step == (i, j)) {
                output.push_str(&format!("[{:2}]", index));
            } else {
                output.push_str(&format!(" {:2} ", value));
            }
        }
        output.push('\n');
    }
    output.push_str(&format!("Captured Points: "));

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

    output.push_str(&format!("Path Score {:2}", path.total_cost));
    log::info!("Path on grid: {}", output);
}
