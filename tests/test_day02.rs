use aoc_2023::solutions::day02::{puzzle_1, puzzle_2};

const EXAMPLE_INPUT: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

#[test]
fn example_1_puzzle_1() {
    env_logger::init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(8));
}

#[test]
fn example_1_puzzle_2() {
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(2286));
}
