use aoc_2023::solutions::day12::puzzle_1;

const EXAMPLE_INPUT_1: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

#[test]
fn puzzle_1_example_1() {
    let _ = env_logger::try_init();
    assert_eq!(puzzle_1(self::EXAMPLE_INPUT_1), Ok(21));
}
