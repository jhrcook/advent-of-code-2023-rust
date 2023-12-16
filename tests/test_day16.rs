use aoc_2023::solutions::day16::{puzzle_1, puzzle_2};

const EXAMPLE_INPUT_1: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

#[test]
fn puzzle_1_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1), Ok(46));
}

#[test]
fn puzzle_2_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_1), Ok(51));
}
