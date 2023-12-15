mod new_day;

use aoc_2023::{run_all, run_day};
use clap::{Parser, Subcommand};
use std::time::Instant;
/// Simple program to greet a person
#[derive(Debug, Parser)]
#[command(author, version, about="Advent of Code 2023 command line interface.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command()]
    Run {
        #[arg(default_value_t = String::from("puzzle-input"), help="Directory with input data files.")]
        data_dir: String,
        #[arg(
            short,
            long,
            help = "Specific day to execute (runs all if not specified, default)."
        )]
        day: Option<u32>,
        #[arg(
            short,
            long,
            help = "Do not run any puzzles (to estimate start-up time)."
        )]
        empty: bool,
    },
    #[command()]
    New {
        #[arg()]
        title: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::New { title } => {
            match new_day::make_new_day_files(&title) {
                Ok(()) => println!("New day templates created. Good luck!"),
                Err(e) => panic!("Failed to create new file templates:\n  {:?}.", e),
            };
        }
        Command::Run {
            data_dir,
            day,
            empty,
        } => {
            let start = Instant::now();
            if empty {
                println!("Empty run.");
                return;
            }
            match day {
                Some(d) => {
                    println!("Running puzzle {}.", d);
                    run_day(&data_dir, &d).unwrap();
                }
                None => {
                    println!("Running all puzzles.");
                    run_all(&data_dir);
                }
            };
            let duration = start.elapsed();
            println!("Done! 🎉 -- Elapsed time: {:?}", duration);
        }
    }
}
