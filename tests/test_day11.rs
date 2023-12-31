use aoc_2023::solutions::day11::{puzzle_1, puzzle_2};

const EXAMPLE_INPUT_1: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

#[test]
fn puzzle_1_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1), 374);
}

#[test]
fn puzzle_2_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_1, 1), 374);
}

// const EXAMPLE_INPUT_2: &str = "
// .....
// .#...
// .....
// ....#
// ";

// #[test]
// fn puzzle_2_example_2() {
//     let _ = env_logger::try_init();
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT_2, 0), 5);
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT_2, 1), 8);
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT_2, 2), 11);
//     assert_eq!(puzzle_2(self::EXAMPLE_INPUT_2, 10), 3 * 10 + 5);
// }

#[test]
fn puzzle_2_example_3() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_1, 10), 1030);
}

#[test]
fn puzzle_2_example_4() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_1, 100), 8410);
}

const EX1_EXPECTED_OUT_X1: &str = "
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
";

#[test]
fn puzzle_2_example_5() {
    let _ = env_logger::try_init();
    assert_eq!(
        puzzle_2(self::EXAMPLE_INPUT_1, 1),
        puzzle_2(self::EX1_EXPECTED_OUT_X1, 0)
    );
}

const EX1_EXPECTED_OUT_X10: &str = "
............#........................
.........................#...........
#....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
........................#............
.#...................................
....................................#
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.........................#...........
#............#.......................
";

#[test]
fn puzzle_2_example_6() {
    let _ = env_logger::try_init();
    assert_eq!(
        puzzle_2(self::EXAMPLE_INPUT_1, 10),
        puzzle_2(self::EX1_EXPECTED_OUT_X10, 0),
    );
}
