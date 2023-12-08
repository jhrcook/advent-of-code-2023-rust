use aoc_2023::solutions::day08::puzzle_1;

const EXAMPLE_INPUT_1: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

#[test]
fn example_1_puzzle_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1), Ok(2));
}

const EXAMPLE_INPUT_2: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[test]
fn example_2_puzzle_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_2), Ok(6));
}

// #[test]
// fn example_1_puzzle_2() {
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(5905));
// }
