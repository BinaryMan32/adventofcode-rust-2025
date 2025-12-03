use advent_of_code::{Named, Runner, create_runner, named};
use std::str::Lines;

fn parse_batteries(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn max_battery_joltage(batteries: &[u8], count: usize) -> Option<u64> {
    let count = count - 1;
    let max_digit = batteries[..batteries.len() - count].iter().max()?;
    if count > 0 {
        let max_index = batteries.iter().position(|b| b == max_digit)?;
        max_battery_joltage(&batteries[(max_index + 1)..], count)
            .map(|joltage| joltage + (*max_digit as u64) * 10u64.pow(count as u32))
    } else {
        Some(*max_digit as u64)
    }
}

fn part1(input: Lines) -> String {
    input
        .map(parse_batteries)
        .map(|batteries| max_battery_joltage(&batteries, 2).unwrap_or(0))
        .sum::<u64>()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .map(parse_batteries)
        .map(|batteries| max_battery_joltage(&batteries, 12).unwrap_or(0))
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
    use rstest::rstest;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "357");
        verify!(part2, input, "3121910778619");
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_max_battery_joltage_2(#[case] batteries: &str, #[case] expected_joltage: u64) {
        let batteries = parse_batteries(batteries);
        assert_eq!(max_battery_joltage(&batteries, 2), Some(expected_joltage));
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_max_battery_joltage_12(#[case] batteries: &str, #[case] expected_joltage: u64) {
        let batteries = parse_batteries(batteries);
        assert_eq!(max_battery_joltage(&batteries, 12), Some(expected_joltage));
    }
}
