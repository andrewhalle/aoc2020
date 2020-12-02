use aoc2020::days::*;

fn print_usage() -> ! {
    eprintln!(
        "USAGE: aoc2020 [day]\n\
         day - the day to run ('day01', 'day02')"
    );

    std::process::exit(1);
}

fn main() {
    let args = std::env::args();
    let day = args.skip(1).next().unwrap_or_else(|| {
        print_usage();
    });

    let input = std::fs::read_to_string(format!("inputs/{}.input", day))
        .expect(&format!("input file not found for {}", day));
    match day.as_str() {
        "day01" => println!("{:?}", day01(&input)),
        "day02" => println!("{:?}", day02(&input)),
        _ => print_usage(),
    }
}
