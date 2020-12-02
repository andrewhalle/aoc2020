use crate::Solution;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct PasswordEntry {
    num1: u32,
    num2: u32,
    letter: char,
    password: String,
}

enum PasswordPolicy {
    OccurrenceRange,
    PositionXor,
}

impl PasswordEntry {
    fn is_valid(&self, policy: PasswordPolicy) -> bool {
        match policy {
            PasswordPolicy::OccurrenceRange => {
                let mut count = 0;
                for c in self.password.chars() {
                    if c == self.letter {
                        count += 1;
                    }
                }

                count >= self.num1 && count <= self.num2
            }
            PasswordPolicy::PositionXor => {
                let mut count = 0;

                if self.letter
                    == self
                        .password
                        .chars()
                        .nth((self.num1 - 1) as usize)
                        .expect("not a valid password entry")
                {
                    count += 1;
                }
                if self.letter
                    == self
                        .password
                        .chars()
                        .nth((self.num2 - 1) as usize)
                        .expect("not a valid password entry")
                {
                    count += 1;
                }

                count == 1
            }
        }
    }
}

/// Parse a password entry from a string slice.
///
/// min - max letter: password
fn parse_password_entry(input: &str) -> PasswordEntry {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)[\r\n]*$").unwrap();
    }

    let cap = RE
        .captures_iter(input)
        .next()
        .expect("unable to parse password entry from string");

    PasswordEntry {
        num1: cap[1].parse().expect("not a valid password entry"),
        num2: cap[2].parse().expect("not a valid password entry"),
        letter: cap[3].parse().expect("not a valid password entry"),
        password: String::from(&cap[4]),
    }
}

pub fn day02(input: &str) -> Solution<u32> {
    let mut retval = Solution { part1: 0, part2: 0 };

    retval.part1 = input
        .lines()
        .map(|l| parse_password_entry(l))
        .filter(|p| p.is_valid(PasswordPolicy::OccurrenceRange))
        .collect::<Vec<PasswordEntry>>()
        .len() as u32;

    retval.part2 = input
        .lines()
        .map(|l| parse_password_entry(l))
        .filter(|p| p.is_valid(PasswordPolicy::PositionXor))
        .collect::<Vec<PasswordEntry>>()
        .len() as u32;

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let solution = day02(include_str!("../../inputs/day02.input"));
        assert_eq!(solution.part1, 477);
        assert_eq!(solution.part2, 686);
    }
}
