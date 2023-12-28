use aoc_2023::solutions::day17::{puzzle_1, puzzle_2};

const EXAMPLE_INPUT_1: &str = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

#[test]
fn puzzle_1_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1), Ok(102));
}

#[test]
fn puzzle_2_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_1), Ok(94));
}

const EXAMPLE_INPUT_2: &str = "
111111111111
999999999991
999999999991
999999999991
999999999991
";

#[test]
fn puzzle_2_example_2() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_2(self::EXAMPLE_INPUT_2), Ok(71));
}
