use advent_of_code::{Named, Runner, create_runner, named};
use std::str::Lines;

fn parse_batteries(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn max_battery_joltage(batteries: &[u8]) -> u32 {
    let max_first = batteries[..batteries.len() - 1].iter().max().unwrap();
    let max_first_index = batteries.iter().position(|b| b == max_first).unwrap();
    let max_second = batteries[(max_first_index + 1)..].iter().max().unwrap();
    (batteries[max_first_index] * 10 + max_second) as u32
}

fn part1(input: Lines) -> String {
    input
        .map(parse_batteries)
        .map(|batteries| max_battery_joltage(&batteries))
        .sum::<u32>()
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
    use rstest::rstest;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "357");
        verify!(part2, input, "0");
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_max_battery_joltage(#[case] batteries: &str, #[case] expected_joltage: u32) {
        let batteries = parse_batteries(batteries);
        assert_eq!(max_battery_joltage(&batteries), expected_joltage);
    }
}
