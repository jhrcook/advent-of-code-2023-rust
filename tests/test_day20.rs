use aoc_2023::solutions::day20::puzzle_1;

const EXAMPLE_INPUT_1: &str = "
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

const EXAMPLE_INPUT_2: &str = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

#[test]
fn puzzle_1_test_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1, 1), Ok(32));
}

#[test]
fn puzzle_1_test_2() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1, 1000), Ok(32000000));
}

#[test]
fn puzzle_1_test_3() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_2, 1000), Ok(11687500));
}
