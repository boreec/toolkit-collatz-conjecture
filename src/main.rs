mod core;
mod plotter;

use crate::core::CollatzSequence;
use crate::plotter::Plotter;
use clap::Parser;

#[derive(Debug, Default, Parser)]
struct Args {
    #[arg(short, long)]
    start: u128,

    #[arg(short, long)]
    plot: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let collatz_sequence = CollatzSequence::new(args.start);

    if args.plot {
        let plotter = Plotter::new();
        return plotter.plot(&collatz_sequence);
    }
    Ok(())
}
