# collatz-conjecture

This project is a WIP and is not meant to be used yet.

# Installation

## Additional dependencies

- for plotting: pkg-config libfreetype6-dev libfontconfig1-dev

# Usage

The argument `--start N` with `N` being an unsigned integer greater than 0 is 
mandatory to execute the program.

To create a plot image, simply add `--plot`. The generated image's name is
`sequence_from_N.png`, with N being the starting value.

```terminal
$ cargo run -- --start 10 && display sequence_from_10.png
```

