use clap::Parser;

/// CLI Parser to configure the planner
#[derive(Parser)]
#[command(
    author = "Deniz Hofmeister",
    version = "0.1.0",
    about = "A CLI tool for grid-based simulations",
    long_about = None
)]
pub struct Cli {
    /// Grid size
    #[arg(short = 'n', long, default_value = "5")]
    pub size: usize,

    /// Discrete time steps
    #[arg(short = 't', long, default_value = "32")]
    pub time_steps: usize,

    /// Max duration in ms
    #[arg(short = 'T', long, default_value = "100")]
    pub max_duration: usize,

    /// Starting positions x
    #[arg(short = 'x', long, value_parser = parse_position, number_of_values = 1, action = clap::ArgAction::Append)]
    pub pos_x: Vec<usize>,

    /// Starting positions y
    #[arg(short = 'y', long, value_parser = parse_position, number_of_values = 1, action = clap::ArgAction::Append)]
    pub pos_y: Vec<usize>,

    /// Source grid
    #[arg(short = 'g', long, default_value = "GRID_S")]
    pub grid: String,
}

pub fn parse_position(s: &str) -> Result<usize, String> {
    s.parse().map_err(|_| format!("Invalid number: {}", s))
}
