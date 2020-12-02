pub mod days;

#[derive(Debug)]
pub struct Solution<T, U = T> {
    part1: T,
    part2: U,
}
