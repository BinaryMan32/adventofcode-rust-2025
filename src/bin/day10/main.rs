use advent_of_code::{Named, Runner, create_runner, named};
use std::{
    collections::VecDeque,
    str::{FromStr, Lines},
};

struct Machine {
    target_lights: u16,
    buttons: Vec<u16>,
    joltage_requirements: Vec<u16>,
}

impl Machine {
    fn parse_target_lights(s: &str) -> u16 {
        if s.chars().nth(0).unwrap() != '[' {
            panic!("incorrect enclosing characters in {s}")
        }
        s[1..s.len()-1].chars().enumerate().map(|(num, c)| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!("unexpected char {c}"),
        } << num)
        .fold(0, |a, b| a | b)
    }

    fn parse_button(s: &str) -> u16 {
        if s.chars().nth(0).unwrap() != '(' {
            panic!("incorrect enclosing characters in {s}")
        }
        s[1..s.len() - 1]
            .split(',')
            .map(|n| n.parse::<u8>().expect("integer"))
            .map(|light| 1 << light)
            .fold(0, |a, b| a | b)
    }

    fn parse_joltage_requirements(s: &str) -> Vec<u16> {
        if s.chars().nth(0).unwrap() != '{' {
            panic!("incorrect enclosing characters in {s}")
        }
        s[1..s.len() - 1]
            .split(',')
            .map(|n| n.parse::<u16>().expect("integer"))
            .collect()
    }

    fn fewest_presses(&self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back(MachineState::new());
        while let Some(state) = queue.pop_front() {
            if state.lights == self.target_lights {
                return state.presses as usize;
            }
            for button in self.buttons.iter() {
                queue.push_back(state.press_button(*button));
            }
        }
        panic!("unable to find solution")
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let target_lights =
            Machine::parse_target_lights(parts.next().ok_or("expected first element")?);
        let joltage_requirements =
            Machine::parse_joltage_requirements(parts.next_back().ok_or("expected last element")?);
        let buttons = parts.map(Machine::parse_button).collect();
        Ok(Machine {
            target_lights,
            buttons,
            joltage_requirements,
        })
    }
}

struct MachineState {
    lights: u16,
    presses: u16,
}

impl MachineState {
    fn new() -> Self {
        let lights = 0;
        let presses = 0;
        Self { lights, presses }
    }

    fn press_button(&self, button_lights: u16) -> Self {
        let lights = self.lights ^ button_lights;
        let presses = self.presses + 1;
        Self { lights, presses }
    }
}

fn part1(input: Lines) -> String {
    input
        .map(|line| line.parse::<Machine>().expect("valid machine"))
        .map(|machine| machine.fewest_presses())
        .sum::<usize>()
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
        verify!(part1, input, "7");
        verify!(part2, input, "0");
    }

    #[test]
    fn test_fewest_presses() {
        let input = include_str!("example.txt");
        let fewest: Vec<_> = input
            .lines()
            .map(|line| line.parse::<Machine>().expect("valid machine"))
            .map(|machine| machine.fewest_presses())
            .collect();
        assert_eq!(fewest, [2, 3, 2]);
    }
}
