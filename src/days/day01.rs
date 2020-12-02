use crate::Solution;
use std::collections::HashSet;

/// Finds two items in a HashSet of u32 which sum to n.
fn find_two_sum_n(nums: &HashSet<u32>, n: u32) -> Option<(u32, u32)> {
    for i in nums.iter().filter(|x| **x <= n) {
        let j = n - i;
        if nums.contains(&j) {
            return Some((*i, j));
        }
    }

    None
}

pub fn day01(input: &str) -> Solution<u32> {
    let mut retval = Solution { part1: 0, part2: 0 };

    let nums: HashSet<u32> = input
        .lines()
        .map(|l| l.parse().expect("not a number"))
        .collect();

    let (i, j) = find_two_sum_n(&nums, 2020).expect("no two values sum to 2020");
    retval.part1 = i * j;

    let (i, j, k) = nums
        .iter()
        .map(|x| (x, find_two_sum_n(&nums, 2020 - *x)))
        .filter(|(_, o)| o.is_some())
        .map(|(i, o)| (i, o.unwrap().0, o.unwrap().1))
        .next()
        .expect("no three values sum to 2020");
    retval.part2 = i * j * k;

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() {
        let solution = day01(include_str!("../../inputs/day01.input"));
        assert_eq!(solution.part1, 545379);
        assert_eq!(solution.part2, 257778836);
    }
}
