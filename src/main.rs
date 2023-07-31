use clap::Parser;

#[derive(Debug, Default, Parser)]
struct Args {
    #[clap(short)]
    n: Option<u128>   
}

fn collatz_sequence(n: u128) -> u128 {
    if n == 1 {
        println!("{}", n);
        return 1;
    }
    print!("{},", n);
    if n % 2 == 0 {
        return collatz_sequence(n / 2);
    }
    return collatz_sequence(3 * n + 1);
}

fn main() {
    let args = Args::parse();

    if let Some(n) = args.n {
        collatz_sequence(n);
    }
}

