use advent_of_code_2023::day::Day;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    day: String,
    puzzle_filename: String,
}

fn main() {
    let cli = Arguments::parse();
    let day: Day = cli.day.parse().unwrap();
    let result = day.run(&cli.puzzle_filename).unwrap();
    println!("{}", result);
}
