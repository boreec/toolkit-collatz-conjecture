mod core;

use clap::Parser;
use crate::core::CollatzSequence;


#[derive(Debug, Default, Parser)]
struct Args {
    #[clap(short)]
    n: Option<u128>   
}

fn main() {
    let args = Args::parse();

    if let Some(n) = args.n {
        let collatz_sequence = CollatzSequence::new(n);
        println!("{:?}", collatz_sequence);
    }
}

