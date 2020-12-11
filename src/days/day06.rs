use crate::Solution;
use std::collections::{HashMap, HashSet};

// returns a HashSet of questions to which everyone answered yes
fn parse_group_everyone(input: &str) -> HashSet<char> {
    let mut people_in_group = 1;
    let mut char_counts = HashMap::new();
    for c in input.trim().chars() {
        if c == '\n' {
            people_in_group += 1;
        } else {
            let counter = char_counts.entry(c).or_insert(0);
            *counter += 1;
        }
    }

    let mut retval = HashSet::new();
    for (question, count) in char_counts.iter() {
        if *count == people_in_group {
            retval.insert(*question);
        }
    }

    retval
}

// returns a HashSet of questions to which anyone answered yes
fn parse_group_anyone(input: &str) -> HashSet<char> {
    input.chars().filter(|c| *c != '\n').collect()
}

pub fn day06(input: &str) -> Solution<u32> {
    let groups = input.split("\n\n");

    Solution {
        part1: groups
            .clone()
            .map(parse_group_anyone)
            .fold(0, |accum, curr| accum + curr.len() as u32),
        part2: groups
            .clone()
            .map(parse_group_everyone)
            .fold(0, |accum, curr| accum + curr.len() as u32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06() {
        let solution = day06(include_str!("../../inputs/day06.input"));
        assert_eq!(solution.part1, 6630);
        assert_eq!(solution.part2, 3437);
    }
}
