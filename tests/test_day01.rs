// use aoc_2023::day01::{puzzle_1, puzzle_2};
use aoc_2023::solutions::day01::{puzzle_1, puzzle_2};

const EXAMPLE_INPUT: &str = "
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

const EXAMPLE_INPUT_2: &str = "
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    ";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(142));
}

#[test]
fn example_1_puzzle_2() {
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_2), Ok(281));
}
