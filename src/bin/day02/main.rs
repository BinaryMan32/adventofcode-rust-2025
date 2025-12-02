use advent_of_code::{Named, Runner, create_runner, named};
use num::traits::Euclid;
use std::collections::HashSet;
use std::iter::successors;
use std::ops::RangeInclusive;
use std::str::{FromStr, Lines};

type IdRangeNumber = u64;

struct IdRange {
    range: RangeInclusive<IdRangeNumber>,
}

impl IdRange {
    fn num_digits(n: IdRangeNumber) -> u32 {
        n.checked_ilog10().unwrap_or(0) + 1
    }

    fn next_invalid_part(start: IdRangeNumber, num_parts: usize) -> IdRangeNumber {
        let digits = Self::num_digits(start);
        let (part_digits, remainder) = digits.div_rem_euclid(&(num_parts as u32));
        if remainder != 0 {
            (10 as IdRangeNumber).pow(part_digits)
        } else {
            let candidate = start / (10 as IdRangeNumber).pow(part_digits * (num_parts - 1) as u32);
            if Self::invalid_from_part(candidate, num_parts) < start {
                candidate + 1
            } else {
                candidate
            }
        }
    }

    fn invalid_from_part(part: IdRangeNumber, num_parts: usize) -> IdRangeNumber {
        let digits = Self::num_digits(part);
        let factor = (10 as IdRangeNumber).pow(digits);
        successors(Some(1), |n| Some(n * factor))
            .take(num_parts)
            .map(|n| n * part)
            .sum()
    }

    fn invalid_ids(&self, num_parts: usize) -> impl Iterator<Item = IdRangeNumber> {
        (Self::next_invalid_part(*self.range.start(), num_parts)..)
            .map(move |part| Self::invalid_from_part(part, num_parts))
            .take_while(|id| id <= self.range.end())
    }

    fn invalid_ids_any(&self) -> HashSet<IdRangeNumber> {
        let digits = Self::num_digits(*self.range.end());
        (2..=digits as usize)
            .flat_map(|num_parts| self.invalid_ids(num_parts))
            .collect()
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
        .map(|id_range| id_range.invalid_ids(2).sum::<IdRangeNumber>())
        .sum::<IdRangeNumber>()
        .to_string()
}

fn part2(mut input: Lines) -> String {
    parse_id_ranges(input.next().unwrap())
        .into_iter()
        .map(|id_range| {
            id_range
                .invalid_ids_any()
                .into_iter()
                .sum::<IdRangeNumber>()
        })
        .sum::<IdRangeNumber>()
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
        verify!(part1, input, "1227775554");
        verify!(part2, input, "4174379265");
    }

    #[test]
    fn test_next_invalid_part_2() {
        assert_eq!(IdRange::next_invalid_part(0, 2), 1);
        assert_eq!(IdRange::next_invalid_part(11, 2), 1);
        assert_eq!(IdRange::next_invalid_part(12, 2), 2);
        assert_eq!(IdRange::next_invalid_part(22, 2), 2);
        assert_eq!(IdRange::next_invalid_part(23, 2), 3);
        assert_eq!(IdRange::next_invalid_part(95, 2), 9);
        assert_eq!(IdRange::next_invalid_part(998, 2), 10);
        assert_eq!(IdRange::next_invalid_part(1188511880, 2), 11885);
        assert_eq!(IdRange::next_invalid_part(446443, 2), 446);
        assert_eq!(IdRange::next_invalid_part(38593856, 2), 3859);
    }

    #[test]
    fn test_next_invalid_part_3() {
        assert_eq!(IdRange::next_invalid_part(0, 3), 1);
        assert_eq!(IdRange::next_invalid_part(11, 3), 1);
        assert_eq!(IdRange::next_invalid_part(99, 3), 1);
        assert_eq!(IdRange::next_invalid_part(100, 3), 1);
        assert_eq!(IdRange::next_invalid_part(500, 3), 5);
        assert_eq!(IdRange::next_invalid_part(998, 3), 9);
        assert_eq!(IdRange::next_invalid_part(118851188511880, 3), 11885);
        assert_eq!(IdRange::next_invalid_part(446443446, 3), 446);
    }

    #[test]
    fn test_invalid_from_part_2() {
        assert_eq!(IdRange::invalid_from_part(0, 2), 0);
        assert_eq!(IdRange::invalid_from_part(1, 2), 11);
        assert_eq!(IdRange::invalid_from_part(9, 2), 99);
        assert_eq!(IdRange::invalid_from_part(10, 2), 1010);
        assert_eq!(IdRange::invalid_from_part(12, 2), 1212);
        assert_eq!(IdRange::invalid_from_part(99, 2), 9999);
        assert_eq!(IdRange::invalid_from_part(3859, 2), 38593859);
    }

    #[test]
    fn test_invalid_from_part_3() {
        assert_eq!(IdRange::invalid_from_part(0, 3), 0);
        assert_eq!(IdRange::invalid_from_part(1, 3), 111);
        assert_eq!(IdRange::invalid_from_part(9, 3), 999);
        assert_eq!(IdRange::invalid_from_part(10, 3), 101010);
        assert_eq!(IdRange::invalid_from_part(12, 3), 121212);
        assert_eq!(IdRange::invalid_from_part(99, 3), 999999);
        assert_eq!(IdRange::invalid_from_part(3859, 3), 385938593859);
    }
}
