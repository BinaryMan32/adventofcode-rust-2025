use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::str::Lines;

struct Grid {
    rows: Vec<Vec<bool>>,
}

const NEIGHBORS: &[(isize, isize)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Grid {
    fn parse(input: Lines) -> Self {
        let rows = input
            .map(|line| line.chars().map(|c| c == '@').collect_vec())
            .collect();
        Self { rows }
    }

    fn get(&self, x: isize, y: isize) -> Option<bool> {
        if x >= 0 && y >= 0 {
            self.rows
                .get(y as usize)
                .and_then(|row| row.get(x as usize).copied())
        } else {
            None
        }
    }

    fn neighbor_rolls(&self, x: usize, y: usize) -> usize {
        NEIGHBORS
            .iter()
            .filter(|(nx, ny)| self.get(x as isize + nx, y as isize + ny) == Some(true))
            .count()
    }

    fn accessible_rolls(&self) -> usize {
        self.rows
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(x, is_roll)| **is_roll && self.neighbor_rolls(*x, y) < 4)
                    .count()
            })
            .sum()
    }
}

fn part1(input: Lines) -> String {
    Grid::parse(input).accessible_rolls().to_string()
}

fn part2(input: Lines) -> String {
    input.take(0).count().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let runner: &Runner = create_runner!();
    runner.run(named!(part1), input);
    runner.run(named!(part2), input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::verify;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "13");
        verify!(part2, input, "0");
    }
}
