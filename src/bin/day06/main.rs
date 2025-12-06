use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::str::{FromStr, Lines};

enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(format!("Invalid operation: '{}'", s)),
        }
    }
}

struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn new(operation: Operation) -> Self {
        Self {
            numbers: Vec::new(),
            operation,
        }
    }

    fn add_number(mut self, number: u64) -> Self {
        self.numbers.push(number);
        self
    }

    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multiply => self.numbers.iter().product(),
        }
    }
}

fn parse_problems(mut input: Lines) -> Vec<Problem> {
    let problems = input
        .next_back()
        .expect("at least one line")
        .split_whitespace()
        .map(|s| s.trim().parse::<Operation>().expect("valid operation"))
        .map(Problem::new)
        .collect_vec();
    input.fold(problems, |problems, line| {
        line.split_whitespace()
            .map(|s| s.parse::<u64>().expect("valid number"))
            .zip_eq(problems)
            .map(|(number, problem)| problem.add_number(number))
            .collect_vec()
    })
}

fn part1(input: Lines) -> String {
    parse_problems(input)
        .into_iter()
        .map(|problem| problem.solve())
        .sum::<u64>()
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
        verify!(part1, input, "4277556");
        verify!(part2, input, "0");
    }
}
