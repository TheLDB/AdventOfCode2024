mod solutions;
mod utils;

use clap::Parser;
use solutions::{
    day_1::solution::D1, day_2::solution::D2, day_3::solution::D3, day_4::solution::D4,
    day_5::solution::D5, day_6::solution::D6,
};
use utils::runner::Runner;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u8>,
}

fn main() {
    let solutions: Vec<Box<dyn Runner>> = vec![
        Box::new(D1 {}),
        Box::new(D2 {}),
        Box::new(D3 {}),
        Box::new(D4 {}),
        Box::new(D5 {}),
        Box::new(D6 {}),
    ];

    let cli = Cli::parse();

    let solution = match cli.day {
        Some(day) => solutions
            .iter()
            .find(|s| s.name().1 == day.into())
            .unwrap_or_else(|| panic!("Day {} not implemented!", day)),
        None => solutions.last().unwrap(),
    };

    let (year, day) = solution.name();
    println!("Running day {} ({})", day, year);

    // Benchmark part one
    let start = std::time::Instant::now();
    if let Some(ans) = solution.part_one() {
        let duration = start.elapsed();
        println!("Solution for part one: {} (took {:?})", ans, duration);
    } else {
        println!("No solution for part one!");
    }

    // Benchmark part two
    let start = std::time::Instant::now();
    if let Some(ans) = solution.part_two() {
        let duration = start.elapsed();
        println!("Solution for part two: {} (took {:?})", ans, duration);
    } else {
        println!("No solution for part two!");
    }
}
