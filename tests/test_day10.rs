use aoc_2023::solutions::day10::puzzle_1;

const EXAMPLE_INPUT_1: &str = "
.....
.S-7.
.|.|.
.L-J.
.....
";

#[test]
fn puzzle_1_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1), Ok(4));
}

const EXAMPLE_INPUT_2: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

#[test]
fn puzzle_1_example_2() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_2), Ok(8));
}

// #[test]
// fn example_1_puzzle_2() {
//     let _ = env_logger::try_init();
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT_1), Ok(2));
// }
