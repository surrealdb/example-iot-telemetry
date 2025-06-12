use clap::Parser;

use crate::app::App;

mod app;
mod event;
mod queries;
mod readings;
mod sensors;
mod ui;
mod utils;

/// A simple Rust program that prints random Perlin noise numbers in concurrent threads.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Number of concurrent sensor_count
    #[clap(short, long, default_value = "4")]
    count: usize,

    /// Delay between each print in milliseconds
    #[clap(short, long, default_value = "1000")]
    delay: u64,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let terminal = ratatui::init();
    let result = App::new(args).run(terminal).await;
    ratatui::restore();
    result
}
