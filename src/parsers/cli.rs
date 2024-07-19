use clap::Parser;

// Avalor A.I. Grid Planner
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

    /// Starting position x
    #[arg(short = 'x', long, default_value = "0")]
    pub pos_x: usize,

    /// Starting position y
    #[arg(short = 'y', long, default_value = "0")]
    pub pos_y: usize,

    /// Source grid
    #[arg(short = 'g', long, default_value = "GRID_S")]
    pub grid: String,
}

pub fn parse_position(s: &str) -> Result<usize, String> {
    s.parse().map_err(|_| format!("Invalid number: {}", s))
}
