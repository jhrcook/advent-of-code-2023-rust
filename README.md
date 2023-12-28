# Advent of Code 2023 solutions in Rust

**My solutions to the [Advent of Code 2023](https://adventofcode.com/2023) using Rust.**

[![advent-of-code](https://img.shields.io/badge/Advent_of_Code-2023-F80046.svg?style=flat)](https://adventofcode.com)
[![rust](https://img.shields.io/badge/Rust-1.74.0-000000.svg?style=flat&logo=rust)](https://www.python.org)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)
[![Cargo Build & Test](https://github.com/jhrcook/advent-of-code-2023-rust/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/jhrcook/advent-of-code-2023-rust/actions/workflows/ci.yaml)

| Day | Code                                                     | Stars |
| ---:| -------------------------------------------------------- |:-----:|
| 1   | [src/solutions/day01.rs](src/solutions/day01.rs)         | ⭐️⭐️   |
| 2   | [src/solutions/day02.rs](src/solutions/day02.rs)         | ⭐️⭐️   |
| 3   | [src/solutions/day03.rs](src/solutions/day03.rs)         | ⭐️⭐️   |
| 4   | [src/solutions/day04.rs](src/solutions/day04.rs)         | ⭐️⭐️   |
| 5   | [src/solutions/day05.rs](src/solutions/day05.rs)         | ⭐️⭐️   |
| 6   | [src/solutions/day06.rs](src/solutions/day06.rs)         | ⭐️⭐️   |
| 7   | [src/solutions/day07.rs](src/solutions/day07.rs)         | ⭐️⭐️   |
| 8   | [src/solutions/day08.rs](src/solutions/day08.rs)         | ⭐️⭐️   |
| 9   | [src/solutions/day09.rs](src/solutions/day09.rs)         | ⭐️⭐️   |
| 10  | [src/solutions/day10.rs](src/solutions/day10.rs)         | ⭐️    |
| 11  | [src/solutions/day11.rs](src/solutions/day11.rs)         | ⭐️⭐️   |
| 12  | [src/solutions/day12.rs](src/solutions/day12.rs)         | ⭐️⭐️   |
| 13  | [src/solutions/day13.rs](src/solutions/day13.rs)         | ⭐️⭐️   |
| 14  | [src/solutions/day14.rs](src/solutions/day14.rs)         | ⭐️⭐️   |
| 15  | [src/solutions/day15.rs](src/solutions/day15.rs)         | ⭐️⭐️   |
| 16  | [src/solutions/day16.rs](src/solutions/day16.rs)         | ⭐️⭐️   |
| 17  | [src/solutions/day17.rs](src/solutions/day17.rs)         | ⭐️⭐️   |
<!-- | 18  | [src/solutions/day18.rs](src/solutions/day18.rs)         | ⭐️⭐️   | -->
<!-- | 19  | [src/solutions/day19.rs](src/solutions/day19.rs)         | ⭐️⭐️   | -->
<!-- | 20  | [src/solutions/day20.rs](src/solutions/day20.rs)         | ⭐️⭐️   | -->
<!-- | 21  | [src/solutions/day21.rs](src/solutions/day21.rs)         | ⭐️⭐️   | -->
<!-- | 22  | [src/solutions/day22.rs](src/solutions/day22.rs)         | ⭐️⭐️   | -->
<!-- | 23  | [src/solutions/day23.rs](src/solutions/day23.rs)         | ⭐️⭐️   | -->
<!-- | 24  | [src/solutions/day24.rs](src/solutions/day24.rs)         | ⭐️⭐️   | -->

## Help

Used this Reddit post for Day 12 part 2: <https://www.reddit.com/r/adventofcode/comments/18hbbxe/2023_day_12python_stepbystep_tutorial_with_bonus/>

I implemented the algorithm described in this Reddit [post](https://www.reddit.com/r/adventofcode/comments/18k9ne5/comment/kdqp7jx/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button) for Day 17 ([Python code](https://topaz.github.io/paste/#XQAAAQCcAgAAAAAAAAAzHIoib6py7i/yVWhl9dSCjkM4GPHrtd8B89i40hrslkM0i9r9hxir+33uzZa01Y7HY/oCJMAoqvX6OLOsC224RKbiSlSU1U6sDn8KTPKvoKhtJCgPq4QDeXZl8oKQCyrAOP0g3rHGLO7gde4rqBUfOeNyypDl5CSgMF0ceJifzUGjB1OliVqXbO/Ydsmg77dYyTbWx89UvPTsZiijfyTYH7ybEz1UtsTx6VHFZ5zcAVDl7ClaQ7+4gn7tShBjy8XQ0s9XR6uWqgo3vPVBjj9Bf3UGCWSP8qYw9N4dZcLcnLbZQgRkBbK2s9a0Cl0XXD0ie4lmMxzz3pLE3i7GFFzUEv9/dNRee0hFwDsxRBK7ERsb8Xt+mYS+fyiltY71gJKfELcn9Eu3TJy2kI/k2o4YHLSGL20gXFLoE4CunCJ2f6iLMbyU/+WeeKUMZn4BQ72S3uZ4SAD9wV+H44NtZEq0I6qYBCrLX98ODOc6lTFAPjJWFmvpIv74Evkb))

## Setup

```bash
git clone git@github.com:jhrcook/advent-of-code-2023-rust.git
cd advent-of-code-2023-rust
```

## Execute puzzles

Test:

```bash
cargo check
cargo test
```

Dev build and run:

```bash
cargo run --release
cargo run --release -- --day 1
```

Full install and run:

```bash
cargo install --path .
aoc-2023
aoc-2023 --day 1
```

For estimating start-up time:

```bash
aoc-2023 --empty
```
