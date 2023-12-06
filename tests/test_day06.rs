use aoc_2023::solutions::day06::puzzle_1;

const EXAMPLE_INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(288));
}

// #[test]
// fn example_1_puzzle_2() {
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(46));
// }
