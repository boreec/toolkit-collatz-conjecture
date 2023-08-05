mod benchmark;
mod core;
mod plotter;

use crate::core::CollatzSequence;
use crate::plotter::Plotter;
use benchmark::benchmark;
use clap::Parser;

#[derive(Debug, Default, Parser)]
struct Args {
    #[arg(short = 'b', long = "benchmark")]
    benchmark: Option<u128>,

    #[arg(short = 'p', long = "plot")]
    plot: bool,

    #[arg(short = 's', long = "start")]
    start: Option<u128>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(b) = args.benchmark {
        benchmark(b);
    }

    if let Some(s) = args.start {
        let collatz_sequence = CollatzSequence::new(s);
        println!("{:?}", collatz_sequence);
        if args.plot {
            let plotter = Plotter::new();
            return plotter.plot(&collatz_sequence);
        }
    }

    Ok(())
}
