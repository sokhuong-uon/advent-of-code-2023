use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Advent of Code solutions")]
struct Args {
    /// Positive integer in range [1,25]
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,

    /// Positive integer in range [1,2]
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();
    match (args.day, args.part) {
        (Some(day), Some(part)) => run_solution(day, part),
        _ => {
            eprintln!("Please provide both day and part numbers.");
            eprintln!("Usage: cargo run -- --day <DAY> --part <PART>");
            eprintln!("For more information, run: cargo run -- --help");
            process::exit(1);
        }
    }
}

fn run_solution(day: u8, part: u8) {
    match (day, part) {
        (1, 1) => println!("{}", day01::part1::solution::main()),
        (1, 2) => println!("{}", day01::part2::solution::main()),
        (2, 1) => println!("{}", day02::part1::solution::main()),
        (2, 2) => println!("{}", day02::part2::solution::main()),
        (3, 1) => println!("{}", day03::part1::solution::main()),
        (3, 2) => println!("{}", day03::part2::solution::main()),
        (4, 1) => println!("{}", day04::part1::solution::main()),
        (4, 2) => println!("{}", day04::part2::solution::main()),
        (5, 1) => println!("{}", day05::part1::solution::main()),
        (5, 2) => println!("{}", day05::part2::solution::main()),
        (6, 1) => println!("{}", day06::part1::solution::main()),
        (6, 2) => println!("{}", day06::part2::solution::main()),
        (7, 1) => println!("{}", day07::part1::solution::main()),
        (7, 2) => println!("{}", day07::part2::solution::main()),
        (8, 1) => println!("{}", day08::part1::solution::main()),
        // (8, 2) => println!("{}", day08::part2::solution::main()),
        (9, 1) => println!("{}", day09::part1::solution::main()),
        (9, 2) => println!("{}", day09::part2::solution::main()),
        (10, 1) => println!("{}", day10::part1::solution::main()),
        (10, 2) => println!("{}", day10::part2::solution::main()),
        (11, 1) => println!("{}", day11::part1::solution::main()),
        (11, 2) => println!("{}", day11::part2::solution::main()),
        (12, 1) => println!("{}", day12::part1::solution::main()),
        // (12, 2) => println!("{}", day12::part2::solution::main()),
        (13, 1) => println!("{}", day13::part1::solution::main()),
        (13, 2) => println!("{}", day13::part2::solution::main()),
        (14, 1) => println!("{}", day14::part1::solution::main()),
        (14, 2) => println!("{}", day14::part2::solution::main()),
        (15, 1) => println!("{}", day15::part1::solution::main()),
        (15, 2) => println!("{}", day15::part2::solution::main()),
        (16, 1) => println!("{}", day16::part1::solution::main()),
        (16, 2) => println!("{}", day16::part2::solution::main()),
        (17, 1) => println!("{}", day17::part1::solution::main()),
        (17, 2) => println!("{}", day17::part2::solution::main()),
        (18, 1) => println!("{}", day18::part1::solution::main()),
        (18, 2) => println!("{}", day18::part2::solution::main()),
        // (19, 1) => println!("{}", day19::part1::solution::main()),
        // (19, 2) => println!("{}", day19::part2::solution::main()),
        // (20, 1) => println!("{}", day20::part1::solution::main()),
        // (20, 2) => println!("{}", day20::part2::solution::main()),
        (21, 1) => println!("{}", day21::part1::solution::main()),
        (21, 2) => println!("{}", day21::part2::solution::main()),
        // (22, 1) => println!("{}", day22::part1::solution::main()),
        // (22, 2) => println!("{}", day22::part2::solution::main()),
        // (23, 1) => println!("{}", day23::part1::solution::main()),
        // (23, 2) => println!("{}", day23::part2::solution::main()),
        (24, 1) => println!("{}", day24::part1::solution::main()),
        // (24, 2) => println!("{}", day24::part2::solution::main()),
        // (25, 1) => println!("{}", day25::part1::solution::main()),
        // (25, 2) => println!("{}", day25::part2::solution::main()),
        _ => println!(
            "Solution for Day {} Part {} is not implemented yet.",
            day, part
        ),
    }
}
