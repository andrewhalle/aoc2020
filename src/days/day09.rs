use crate::Solution;
use std::collections::HashMap;

const PREAMBLE_LENGTH: usize = 25;

fn has_sum_property(n: u64, active_set: &HashMap<u64, u32>) -> bool {
    for (k, v) in active_set {
        if *k > n {
            continue;
        }

        if active_set.contains_key(&(n - k)) {
            if *k == n - k {
                if *v > 1 {
                    return true;
                }
            } else {
                return true;
            }
        }
    }

    false
}

fn first_num_without_sum_property(nums: &Vec<u64>) -> u64 {
    let mut active_set = HashMap::new();

    for i in 0..PREAMBLE_LENGTH {
        let counter = active_set.entry(nums[i as usize]).or_insert(0);
        *counter += 1;
    }

    for i in PREAMBLE_LENGTH..nums.len() {
        let num = nums[i as usize];
        if !has_sum_property(num, &active_set) {
            return num;
        }

        let counter = active_set
            .entry(nums[(i - PREAMBLE_LENGTH) as usize])
            .or_insert(0);
        *counter -= 1;
        if *counter == 0 {
            active_set.remove(&nums[(i - PREAMBLE_LENGTH) as usize]);
        }

        let counter = active_set.entry(num).or_insert(0);
        *counter += 1;
    }

    unreachable!();
}

fn find_weakness(hint: u64, nums: &Vec<u64>) -> &[u64] {
    for i in 0..nums.len() {
        let mut sum = 0;
        let mut j = i;
        while sum < hint {
            sum += nums[j];
            j += 1;
        }

        if sum == hint {
            return &nums[i..j + 1];
        }
    }

    unreachable!();
}

pub fn day09(input: &str) -> Solution<u64> {
    let nums = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u64>>();

    let first = first_num_without_sum_property(&nums);
    let weakness = find_weakness(first, &nums);

    Solution {
        part1: first,
        part2: weakness.iter().min().unwrap() + weakness.iter().max().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09() {
        let solution = day09(include_str!("../../inputs/day09.input"));
        assert_eq!(solution.part1, 217430975);
        assert_eq!(solution.part2, 28509180);
    }
}
