use aoc_2023::solutions::day07::puzzle_1;

const EXAMPLE_INPUT: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[test]
fn example_1_puzzle_1() {
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(6440));
}

// #[test]
// fn example_1_puzzle_2() {
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(71503));
// }
