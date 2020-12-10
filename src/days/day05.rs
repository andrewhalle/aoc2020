use crate::Solution;
use std::collections::HashSet;

fn get_seat_id(bsp_description: &str) -> u32 {
    let mut retval = 0;
    let mut mask = 1;

    for c in bsp_description.chars().rev() {
        if c == 'B' || c == 'R' {
            retval |= mask;
        }

        mask <<= 1;
    }

    retval
}

fn find_missing(ids: &HashSet<u32>) -> u32 {
    for id in ids {
        if !ids.contains(&(id + 1)) && ids.contains(&(id + 2)) {
            return id + 1;
        }
    }

    unreachable!();
}

pub fn day05(input: &str) -> Solution<u32> {
    let seat_ids: HashSet<u32> = input.lines().map(get_seat_id).collect();

    Solution {
        part1: *seat_ids.iter().max().unwrap(),
        part2: find_missing(&seat_ids),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05() {
        let solution = day05(include_str!("../../inputs/day05.input"));
        assert_eq!(solution.part1, 0);
        assert_eq!(solution.part2, 0);
    }
}
