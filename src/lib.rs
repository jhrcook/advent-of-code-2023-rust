mod data;
mod math_utils;
pub mod solutions;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Day not yet implemented: {}.", .0)]
    DayNotImplemented(u32),
}

pub fn run_day(data_dir: &str, day: &u32) -> Result<(), Error> {
    match day {
        1 => {
            solutions::day01::main(data_dir);
            Ok(())
        }
        2 => {
            solutions::day02::main(data_dir);
            Ok(())
        }
        3 => {
            solutions::day03::main(data_dir);
            Ok(())
        }
        4 => {
            solutions::day04::main(data_dir);
            Ok(())
        }
        5 => {
            solutions::day05::main(data_dir);
            Ok(())
        }
        6 => {
            solutions::day06::main(data_dir);
            Ok(())
        }
        7 => {
            solutions::day07::main(data_dir);
            Ok(())
        }
        8 => {
            solutions::day08::main(data_dir);
            Ok(())
        }
        9 => {
            solutions::day09::main(data_dir);
            Ok(())
        }
        10 => {
            solutions::day10::main(data_dir);
            Ok(())
        }
        11 => {
            solutions::day11::main(data_dir);
            Ok(())
        }
        12 => {
            solutions::day12::main(data_dir);
            Ok(())
        }
        13 => {
            solutions::day13::main(data_dir);
            Ok(())
        }
        14 => {
            solutions::day14::main(data_dir);
            Ok(())
        }
        15 => {
            solutions::day15::main(data_dir);
            Ok(())
        }
        16 => {
            solutions::day16::main(data_dir);
            Ok(())
        }
        17 => {
            solutions::day17::main(data_dir);
            Ok(())
        }
        18 => {
            solutions::day18::main(data_dir);
            Ok(())
        }
        19 => {
            solutions::day19::main(data_dir);
            Ok(())
        }
        20 => {
            solutions::day20::main(data_dir);
            Ok(())
        }
        // <-- INSERT NEW DAY HERE -->
        _ => Err(Error::DayNotImplemented(*day)),
    }
}

pub fn run_all(data_dir: &str) {
    for day in 1..26 {
        match run_day(data_dir, &day) {
            Ok(()) => continue,
            Err(Error::DayNotImplemented(_)) => break,
        }
    }
}
