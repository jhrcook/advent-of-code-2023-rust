use aoc_2023::solutions::day10::{puzzle_1, puzzle_2};

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

const EXAMPLE_INPUT_3: &str = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

#[test]
fn puzzle_2_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_3), Ok(4));
}

const EXAMPLE_INPUT_4: &str = "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

#[test]
fn puzzle_2_example_2() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_4), Ok(4));
}

const EXAMPLE_INPUT_5: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

#[test]
fn puzzle_2_example_3() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_5), Ok(8));
}

const EXAMPLE_INPUT_6: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

#[test]
fn puzzle_2_example_4() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_6), Ok(10));
}
