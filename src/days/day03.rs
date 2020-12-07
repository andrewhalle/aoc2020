use crate::Solution;

struct Map {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
}

impl Map {
    fn get(&self, coords: (usize, usize)) -> bool {
        self.data[coords.0 * self.cols + coords.1]
    }
}

fn trees_on_slope(map: &Map, row_delta: usize, col_delta: usize) -> u32 {
    let mut curr = (0, 0);
    let mut trees = 0;
    while curr.0 < map.rows - 1 {
        curr.0 += row_delta;

        curr.1 += col_delta;
        curr.1 %= map.cols;

        if map.get(curr) {
            trees += 1;
        }
    }

    trees
}

pub fn day03(input: &str) -> Solution<u32> {
    let rows = input.lines().fold(0, |curr, _| curr + 1);
    let cols = input.split('\n').next().unwrap().len();

    let mut map = vec![false; rows * cols];
    for (i, c) in input.chars().filter(|c| *c != '\n').enumerate() {
        map[i] = c == '#';
    }
    let map = Map {
        rows,
        cols,
        data: map,
    };

    let trees = vec![
        trees_on_slope(&map, 1, 1),
        trees_on_slope(&map, 1, 3),
        trees_on_slope(&map, 1, 5),
        trees_on_slope(&map, 1, 7),
        trees_on_slope(&map, 2, 1),
    ];

    Solution {
        part1: trees[1],
        part2: trees.iter().fold(1, |accum, curr| accum * curr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03() {
        let solution = day03(include_str!("../../inputs/day03.input"));
        assert_eq!(solution.part1, 167);
        assert_eq!(solution.part2, 736527114);
    }
}
