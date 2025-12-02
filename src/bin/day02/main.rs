use advent_of_code::{Named, Runner, create_runner, named};
use num::traits::Euclid;
use std::ops::RangeInclusive;
use std::str::{FromStr, Lines};

type IdRangeNumber = u64;

struct IdRange {
    range: RangeInclusive<IdRangeNumber>,
}

impl IdRange {
    fn next_invalid_part(start: IdRangeNumber) -> IdRangeNumber {
        let digits = start.checked_ilog10().unwrap_or(0) + 1;
        let (half_digits, remainder) = digits.div_rem_euclid(&2);
        if remainder == 1 {
            (10 as IdRangeNumber).pow(half_digits)
        } else {
            let candidate = start / (10 as IdRangeNumber).pow(half_digits);
            if Self::invalid_from_part(candidate) < start {
                candidate + 1
            } else {
                candidate
            }
        }
    }

    fn invalid_from_part(part: IdRangeNumber) -> IdRangeNumber {
        let digits = part.checked_ilog10().unwrap_or(0) + 1;
        part * (10 as IdRangeNumber).pow(digits) + part
    }

    fn invalid_ids(&self) -> impl Iterator<Item = IdRangeNumber> {
        (Self::next_invalid_part(*self.range.start())..)
            .map(Self::invalid_from_part)
            .take_while(|id| id <= self.range.end())
    }

    fn sum_invalid_ids(&self) -> IdRangeNumber {
        self.invalid_ids().sum()
    }
}

impl FromStr for IdRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bounds = s
            .split('-')
            .filter_map(|part| part.parse::<IdRangeNumber>().ok());
        match (bounds.next(), bounds.next()) {
            (Some(start), Some(end)) => Ok(IdRange {
                range: (start..=end),
            }),
            _ => Err(format!("Failed to parse IdRange from '{}'", s)),
        }
    }
}

fn parse_id_ranges(line: &str) -> Vec<IdRange> {
    line.split(',')
        .map(|part| part.parse::<IdRange>().unwrap())
        .collect()
}

fn part1(mut input: Lines) -> String {
    parse_id_ranges(input.next().unwrap())
        .into_iter()
        .map(|id_range| id_range.sum_invalid_ids())
        .sum::<IdRangeNumber>()
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
        verify!(part1, input, "1227775554");
        verify!(part2, input, "0");
    }

    #[test]
    fn test_next_invalid_part() {
        assert_eq!(IdRange::next_invalid_part(0), 1);
        assert_eq!(IdRange::next_invalid_part(11), 1);
        assert_eq!(IdRange::next_invalid_part(12), 2);
        assert_eq!(IdRange::next_invalid_part(22), 2);
        assert_eq!(IdRange::next_invalid_part(23), 3);
        assert_eq!(IdRange::next_invalid_part(95), 9);
        assert_eq!(IdRange::next_invalid_part(998), 10);
        assert_eq!(IdRange::next_invalid_part(1188511880), 11885);
        assert_eq!(IdRange::next_invalid_part(446443), 446);
        assert_eq!(IdRange::next_invalid_part(38593856), 3859);
    }

    #[test]
    fn test_invalid_from_part() {
        assert_eq!(IdRange::invalid_from_part(0), 0);
        assert_eq!(IdRange::invalid_from_part(1), 11);
        assert_eq!(IdRange::invalid_from_part(9), 99);
        assert_eq!(IdRange::invalid_from_part(10), 1010);
        assert_eq!(IdRange::invalid_from_part(12), 1212);
        assert_eq!(IdRange::invalid_from_part(99), 9999);
        assert_eq!(IdRange::invalid_from_part(3859), 38593859);
    }
}
