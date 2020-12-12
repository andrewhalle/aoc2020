use crate::Solution;
use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, HashSet<(u32, &'a str)>>;

struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn rule(&mut self) -> Rule<'a> {
        let color = self.color();
        self.input = self.input.strip_prefix("bags contain ").unwrap();
        match self.input.strip_prefix("no other bags.\n") {
            None => {
                let contains = self.list();
                self.input = self.input.strip_prefix("\n").unwrap();

                Rule { color, contains }
            }
            Some(s) => {
                self.input = s;
                Rule {
                    color,
                    contains: Vec::new(),
                }
            }
        }
    }

    fn color(&mut self) -> &'a str {
        let first_space = self.input.find(" ").unwrap();
        let second_space = self.input[first_space + 1..].find(" ").unwrap();

        let color = &self.input[..first_space + second_space + 1];
        self.input = &self.input[first_space + second_space + 2..];

        color
    }

    fn list(&mut self) -> Vec<(u32, &'a str)> {
        let mut retval = vec![self.atom()];
        while let Some(s) = self.input.strip_prefix(", ") {
            self.input = s;
            retval.push(self.atom());
        }
        self.input = self.input.strip_prefix(".").unwrap();

        retval
    }

    fn atom(&mut self) -> (u32, &'a str) {
        let num = self.num();
        let color = self.color();

        self.input = self.input.strip_prefix("bag").unwrap();
        self.input = match self.input.strip_prefix("s") {
            Some(s) => s,
            None => self.input,
        };

        (num, color)
    }

    fn num(&mut self) -> u32 {
        let mut curr = 0;
        while self.input.chars().nth(curr).unwrap() >= '0'
            && self.input.chars().nth(curr).unwrap() <= '9'
        {
            curr += 1;
        }

        let num = &self.input[..curr];
        self.input = &self.input[curr + 1..];

        num.parse().unwrap()
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Rule<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input == "" {
            None
        } else {
            Some(self.rule())
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    color: &'a str,
    contains: Vec<(u32, &'a str)>,
}

fn could_contain<'a>(graph: &Graph<'a>, entry: &'a str) -> HashSet<(u32, &'a str)> {
    let neighbors = graph.get(entry);
    match neighbors {
        None => HashSet::new(),
        Some(neighbors) => {
            let mut total = neighbors.clone();
            for (_, n1) in neighbors {
                for (_, n2) in could_contain(graph, n1) {
                    total.insert((0, n2));
                }
            }
            total
        }
    }
}

enum EdgeDirection {
    ToContainer,
    ToContained,
}

fn build_graph_from_rules<'a>(parser: Parser<'a>, edge_direction: EdgeDirection) -> Graph<'a> {
    let mut graph = HashMap::new();
    for rule in parser {
        for (num, color) in rule.contains {
            match edge_direction {
                EdgeDirection::ToContainer => {
                    let neighbors = graph.entry(color).or_insert(HashSet::new());
                    neighbors.insert((0, rule.color));
                }
                EdgeDirection::ToContained => {
                    let neighbors = graph.entry(rule.color).or_insert(HashSet::new());
                    neighbors.insert((num, color));
                }
            }
        }
    }

    graph
}

fn contains<'a>(graph: &Graph<'a>, entry: &'a str) -> u32 {
    let neighbors = graph.get(entry);
    match neighbors {
        None => 0,
        Some(neighbors) => {
            let mut total = 0;
            for (num, n) in neighbors {
                total += num;
                total += num * contains(graph, n);
            }
            total
        }
    }
}

pub fn day07(input: &str) -> Solution<u32> {
    let mut retval = Solution { part1: 0, part2: 0 };

    let graph = build_graph_from_rules(Parser { input }, EdgeDirection::ToContainer);
    retval.part1 = could_contain(&graph, "shiny gold").len() as u32;

    let graph = build_graph_from_rules(Parser { input }, EdgeDirection::ToContained);
    retval.part2 = contains(&graph, "shiny gold");

    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day07() {
        let solution = day07(include_str!("../../inputs/day07.input"));
        assert_eq!(solution.part1, 124);
        assert_eq!(solution.part2, 34862);
    }
}
