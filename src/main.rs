use aoc_2023::{run_all, run_day};
use clap::Parser;
use std::time::Instant;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Data directory.
    #[arg(default_value_t = String::from("puzzle-input"))]
    data_dir: String,
    #[arg(short, long)]
    day: Option<usize>,
    #[arg(long)]
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
