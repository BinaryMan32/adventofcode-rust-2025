use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::str::{FromStr, Lines};

struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn rectangle_area_with(&self, other: &Pos) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or(format!("Pos didn't contain at least one ',' \"{}\"", s))?;
        Ok(Pos {
            x: x.parse()
                .map_err(|e| format!("Unable to parse x coord \"{}\" due to: {}", x, e))?,
            y: y.parse()
                .map_err(|e| format!("Unable to parse y coord \"{}\" due to: {}", x, e))?,
        })
    }
}

fn part1(input: Lines) -> String {
    let points = input
        .into_iter()
        .map(|line| line.parse::<Pos>().unwrap())
        .collect_vec();
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.rectangle_area_with(b))
        .max()
        .expect("at least one element")
        .to_string()
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
        verify!(part1, input, "50");
        verify!(part2, input, "0");
    }
}
