use crate::types::{Grid, Path};

pub fn plot_paths(grid: &Grid, paths: &Vec<Path>) {
    let mut output = String::new();
    output.push('\n');

    for i in 0..grid.size {
        for j in 0..grid.size {
            let value = grid.value_at(i, j);
            let mut path_index = None;
            let mut step_index = None;

            for (idx, path) in paths.iter().enumerate() {
                if let Some(pos) = path.steps.iter().position(|&step| step == (i, j)) {
                    path_index = Some(idx);
                    step_index = Some(pos);
                    break;
                }
            }

            match (path_index, step_index) {
                (Some(p_idx), Some(0)) => output.push_str(&format!("[{:X}]", p_idx)),
                (Some(p_idx), Some(s_idx)) => output.push_str(&format!("[{:X}]", p_idx)),
                _ => output.push_str(&format!(" {:2} ", value)),
            }
        }
        output.push('\n');
    }

    output.push_str("Captured Points:\n");

    for (idx, path) in paths.iter().enumerate() {
        output.push_str(&format!("Path {}: ", idx));
        for &(i, j) in &path.steps {
            let value = grid.value_at(i, j);
            output.push_str(&format!("({},{})={} ", i, j, value));
        }
        output.push_str(&format!("Score: {}\n", path.total_cost));
    }

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
