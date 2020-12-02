use aoc2020::days::*;
use std::collections::HashMap;

fn print_usage() -> ! {
    eprintln!(
        "USAGE: aoc2020 [day]\n\
         day - the day to run ('day01', 'day02')"
    );

    std::process::exit(1);
}

fn main() {
    let mut days_handlers = HashMap::new();
    days_handlers.insert(String::from("day01"), day01);

    let args = std::env::args();
    let day = args.skip(1).next().unwrap_or_else(|| {
        print_usage();
    });

    let input = std::fs::read_to_string(format!("inputs/{}.input", day))
        .expect(&format!("input file not found for {}", day));
    let handler = days_handlers.get(&day).unwrap();

    println!("{:?}", handler(&input));
}
