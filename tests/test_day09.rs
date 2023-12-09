use aoc_2023::solutions::day09::puzzle_1;

const EXAMPLE_INPUT_1: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

#[test]
fn example_1_puzzle_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1), Ok(114));
}
