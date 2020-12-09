use aoc2020::days::*;

const DAYS: [&'static str; 4] = ["day01", "day02", "day03", "day04"];

fn print_usage() -> ! {
    eprintln!(
        "USAGE: aoc2020 [day]\n\
         day - the day to run ('day01', 'day02')"
    );

    std::process::exit(1);
}

fn day_main(day: &str) {
    let input = std::fs::read_to_string(format!("inputs/{}.input", day))
        .expect(&format!("input file not found for {}", day));
    println!("Solution for {}", day);
    match day {
        "day01" => println!("{:?}", day01(&input)),
        "day02" => println!("{:?}", day02(&input)),
        "day03" => println!("{:?}", day03(&input)),
        "day04" => println!("{:?}", day04(&input)),
        _ => print_usage(),
    }
}

fn main() {
    let args = std::env::args();
    let day = args.skip(1).next();

    match day {
        Some(day) => day_main(&day),
        None => {
            for (i, day) in DAYS.iter().enumerate() {
                day_main(&day);
                if i != DAYS.len() - 1 {
                    println!();
                }
            }
        }
    }
}
