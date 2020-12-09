use crate::Solution;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

enum Mode {
    RequiredFields,
    ValidFields,
}

fn present_and_between(x: &Option<String>, min: u32, max: u32) -> bool {
    x.as_ref()
        .and_then(|s| s.parse::<u32>().ok())
        .map(|n| n >= min && n <= max)
        .unwrap_or(false)
}

fn height_check(s: &Option<String>) -> bool {
    let inch = s
        .as_ref()
        .and_then(|s| s.strip_suffix("in"))
        .and_then(|s| s.parse::<u32>().ok())
        .map(|n| n >= 59 && n <= 76)
        .unwrap_or(false);

    let cm = s
        .as_ref()
        .and_then(|s| s.strip_suffix("cm"))
        .and_then(|s| s.parse::<u32>().ok())
        .map(|n| n >= 150 && n <= 193)
        .unwrap_or(false);

    inch || cm
}

fn re_check(s: &Option<String>, re: &Regex) -> bool {
    s.as_ref().map(|s| re.is_match(s)).unwrap_or(false)
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn fields_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn fields_valid(&self) -> bool {
        lazy_static! {
            static ref COLOR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref EYE_RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        if !self.fields_present() {
            return false;
        }

        present_and_between(&self.byr, 1920, 2002)
            && present_and_between(&self.iyr, 2010, 2020)
            && present_and_between(&self.eyr, 2020, 2030)
            && height_check(&self.hgt)
            && re_check(&self.hcl, &COLOR_RE)
            && re_check(&self.ecl, &EYE_RE)
            && re_check(&self.pid, &PID_RE)
    }

    fn valid(&self, validity_mode: Mode) -> bool {
        match validity_mode {
            Mode::RequiredFields => self.fields_present(),
            Mode::ValidFields => self.fields_valid(),
        }
    }
}

fn parse_key_value(s: &str) -> (&str, &str) {
    let i = s.find(':').unwrap();
    let (key, rest) = s.split_at(i);
    let val = rest.strip_prefix(":").unwrap();

    (key, val)
}

fn parse_passport(s: &str) -> Passport {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s+").unwrap();
    }

    let mut retval = Passport::new();
    for (key, val) in RE.split(s).filter(|s| *s != "").map(parse_key_value) {
        let val = Some(val.to_owned());
        match key {
            "byr" => retval.byr = val,
            "iyr" => retval.iyr = val,
            "eyr" => retval.eyr = val,
            "hgt" => retval.hgt = val,
            "hcl" => retval.hcl = val,
            "ecl" => retval.ecl = val,
            "pid" => retval.pid = val,
            "cid" => retval.cid = val,
            _ => unreachable!(),
        }
    }

    retval
}

pub fn day04(input: &str) -> Solution<u32> {
    let passports = input.split("\n\n").filter(|s| *s != "").map(parse_passport);

    Solution {
        part1: passports
            .clone()
            .filter(|p| p.valid(Mode::RequiredFields))
            .fold(0, |sum, _| sum + 1),
        part2: passports
            .clone()
            .filter(|p| p.valid(Mode::ValidFields))
            .fold(0, |sum, _| sum + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04() {
        let solution = day04(include_str!("../../inputs/day04.input"));
        assert_eq!(solution.part1, 250);
        assert_eq!(solution.part2, 158);
    }
}
