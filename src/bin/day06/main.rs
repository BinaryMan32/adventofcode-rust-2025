use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::str::{FromStr, Lines};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

fn parse_problems1(mut input: Lines) -> Vec<Problem> {
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
    parse_problems1(input)
        .into_iter()
        .map(|problem| problem.solve())
        .sum::<u64>()
        .to_string()
}

fn parse_problems2(input: Lines) -> Vec<Problem> {
    let lines = input.collect_vec();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let num_digits = lines.len() - 1;
    let mut problems = Vec::new();
    let mut numbers = Vec::<u64>::new();
    for column in (0..max_len).rev() {
        let maybe_number = (0..num_digits)
            .flat_map(|line| lines[line].chars().nth(column))
            .collect::<String>()
            .trim()
            .parse::<u64>()
            .ok();
        if let Some(number) = maybe_number {
            numbers.push(number);
        }
        let maybe_operation = lines
            .last()
            .unwrap()
            .chars()
            .nth(column)
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_string().parse::<Operation>().expect("valid operation"));
        if let Some(operation) = maybe_operation {
            problems.push(Problem { numbers, operation });
            numbers = Vec::new();
        }
    }
    problems
}

fn part2(input: Lines) -> String {
    parse_problems2(input)
        .into_iter()
        .map(|problem| problem.solve())
        .sum::<u64>()
        .to_string()
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
        verify!(part2, input, "3263827");
    }

    #[test]
    fn test_parse_problems2() {
        let input = include_str!("example.txt");
        assert_eq!(
            parse_problems2(input.lines()),
            vec![
                Problem {
                    numbers: vec![4, 431, 623],
                    operation: Operation::Add,
                },
                Problem {
                    numbers: vec![175, 581, 32],
                    operation: Operation::Multiply,
                },
                Problem {
                    numbers: vec![8, 248, 369],
                    operation: Operation::Add,
                },
                Problem {
                    numbers: vec![356, 24, 1],
                    operation: Operation::Multiply,
                },
            ]
        );
    }
}
