use crate::Solution;
use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

struct Machine {
    instructions: Vec<Instruction>,
    acc: i32,
    ip: usize,
}

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(" ");

        let opcode = words.next().unwrap();
        let num = words.next().unwrap().parse().unwrap();

        Ok(match opcode {
            "jmp" => Instruction::Jmp(num),
            "acc" => Instruction::Acc(num),
            "nop" => Instruction::Nop(num),
            _ => unreachable!(),
        })
    }
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Machine {
        Machine {
            instructions,
            acc: 0,
            ip: 0,
        }
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.ip = 0;
    }

    fn run_til_repeat(&mut self, track_candidates: bool) -> (bool, Option<Vec<usize>>) {
        let mut seen = HashSet::new();
        let mut candidates = if track_candidates {
            Some(Vec::new())
        } else {
            None
        };

        while !seen.contains(&self.ip) {
            if self.ip == self.instructions.len() {
                return (true, candidates);
            }

            seen.insert(self.ip);
            let curr = &self.instructions[self.ip];
            match curr {
                Instruction::Acc(n) => {
                    self.acc += n;
                    self.ip += 1;
                }
                Instruction::Jmp(n) => {
                    if candidates.is_some() {
                        candidates.as_mut().unwrap().push(self.ip);
                    }

                    if *n < 0 {
                        self.ip -= n.abs() as usize;
                    } else {
                        self.ip += *n as usize;
                    }
                }
                Instruction::Nop(_) => {
                    if candidates.is_some() {
                        candidates.as_mut().unwrap().push(self.ip);
                    }

                    self.ip += 1;
                }
            }
        }

        (false, candidates)
    }

    fn swap(&mut self, instruction: usize) {
        let instruction = &mut self.instructions[instruction];

        let mut new = match instruction {
            Instruction::Jmp(n) => Instruction::Nop(*n),
            Instruction::Nop(n) => Instruction::Jmp(*n),
            _ => unreachable!(),
        };

        std::mem::swap(instruction, &mut new);
    }
}

pub fn day08(input: &str) -> Solution<i32> {
    let mut retval = Solution { part1: 0, part2: 0 };

    let instructions = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Instruction>>();
    let mut machine = Machine::new(instructions);

    let (_, replacement_candidates) = machine.run_til_repeat(true);
    let replacement_candidates = replacement_candidates.unwrap();
    retval.part1 = machine.acc;

    for candidate in replacement_candidates {
        machine.swap(candidate);
        machine.reset();
        if machine.run_til_repeat(false).0 {
            break;
        }
        machine.swap(candidate);
    }
    retval.part2 = machine.acc;

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08() {
        let solution = day08(include_str!("../../inputs/day08.input"));
        assert_eq!(solution.part1, 1810);
        assert_eq!(solution.part2, 969);
    }
}
