use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::ops::RangeInclusive;
use std::str::Lines;

fn parse_input(mut input: Lines) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let ranges = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            start..=end
        })
        .collect_vec();
    let numbers = input.map(|line| line.parse::<u64>().unwrap()).collect_vec();
    (ranges, numbers)
}

fn part1(input: Lines) -> String {
    let (ranges, numbers) = parse_input(input);
    numbers
        .into_iter()
        .filter(|num| ranges.iter().any(|range| range.contains(num)))
        .count()
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
        verify!(part1, input, "3");
        verify!(part2, input, "0");
    }
}
