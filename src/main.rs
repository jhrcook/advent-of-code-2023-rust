use aoc_2023::{run_all, run_day};
use clap::Parser;
use std::time::Instant;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Data directory.
    #[arg(default_value_t = String::from("puzzle-input"), help="Directory with input data files.")]
    data_dir: String,
    #[arg(
        short,
        long,
        help = "Specific day to execute (runs all if not specified, default)."
    )]
    day: Option<usize>,
    #[arg(
        short,
        long,
        help = "Do not run any puzzles (to estimate start-up time)."
    )]
    empty: bool,
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();
    if args.empty {
        println!("Empty run.");
        return;
    }
    match args.day {
        Some(d) => {
            println!("Running puzzle {}.", d);
            run_day(&args.data_dir, &d);
        }
        None => {
            println!("Running all puzzles.");
            run_all(&args.data_dir);
        }
    };
    let duration = start.elapsed();
    println!("Done! 🎉 -- Elapsed time: {:?}", duration);
}
