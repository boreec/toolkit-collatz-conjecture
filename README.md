# collatz-conjecture

Author: Cyprien Borée

This project is a WIP and is not meant to be used yet.
Todo list:
- Add memoization for efficient bulk sequence computing.
- Improve tree rendering by putting the nodes with a small delta to the same
level.
- Compute steps densitiy for a group of sequences.
- Add github workflow.
- Add markdown style comments to document code.
- Generalize CollatSequence to other forms (5x + 1, negative numbers, etc.)

# Installation

## Additional dependencies

- for plotting: pkg-config libfreetype6-dev libfontconfig1-dev

# Usage

Use `-h` or `--help` for a summary of arguments and options.

## Sequence for a single number
To compute a Collatz sequence from a number n, use `--start` or `-s`:
```terminal
$ cargo run -- --start 10
CollatzSequence { hailstone: [10, 5, 16, 8, 4, 2, 1], starting_number: 10 }
```

## Plotting sequence
To compute a Collatz sequence from a number n, and plot it, use the same command
as above with `--plot` or `-p`. The plot will be saved as `sequence_from_n.png`,
where `n` is the starting number. For example:
```terminal
$ cargo run -- -s 27 -p && display sequence_from_27.png
```
![Collatz Sequence for n = 27](rsc/img/sequence_from_27.png)

## Bottom-up tree
To build the sequence in a reverse order from 1 to given n, use `--tree` arg.
This will create the bottom-up tree and export it in a file named
`tree_to_n.dot`. This file can then be processed into an image with:
```terminal
$ cargo run -- -s 100 --tree && \
  dot -Tpng tree_to_100.dot > tree_to_100.png && \
  display tree_to_100.png
```

<img src="rsc/img/tree_to_1000.png" height="300">

## Benchmarking
Use `-b` or `--benchmark` with a number to compare the efficiency of diverse
methods for computing all steps for the first n numbers. For example:
```terminal
$ cargo run -- -b 100
Time elapsed to compute steps on sequences from 1 to 100: 88.37µs
```
