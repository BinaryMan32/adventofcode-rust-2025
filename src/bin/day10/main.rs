use advent_of_code::{Named, Runner, create_runner, named};
use core::iter::Iterator;
use num::Integer;
use std::{
    collections::{HashMap, VecDeque},
    str::{FromStr, Lines},
};

struct Button {
    values: Vec<u16>,
    mask: u16,
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(0).unwrap() != '(' {
            Err(format!("incorrect enclosing characters in {s}"))
        } else {
            let lights = s[1..s.len() - 1]
                .split(',')
                .map(|n| n.parse::<u16>().expect("integer"))
                .collect::<Vec<u16>>();
            let mask = lights.iter().map(|light| 1 << light).fold(0, |a, b| a | b);
            Ok(Self {
                values: lights,
                mask,
            })
        }
    }
}

struct Machine {
    target_lights: u16,
    buttons: Vec<Button>,
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

    fn parse_joltage_requirements(s: &str) -> Vec<u16> {
        if s.chars().nth(0).unwrap() != '{' {
            panic!("incorrect enclosing characters in {s}")
        }
        s[1..s.len() - 1]
            .split(',')
            .map(|n| n.parse::<u16>().expect("integer"))
            .collect()
    }

    fn fewest_presses_lights(&self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back(MachineStateLights::new());
        while let Some(state) = queue.pop_front() {
            if state.lights == self.target_lights {
                return state.presses as usize;
            }
            for button in self.buttons.iter() {
                queue.push_back(state.press_button(button.mask));
            }
        }
        panic!("unable to find solution")
    }

    fn fewest_presses_joltage(&self) -> Option<usize> {
        self.fewest_presses_joltage_state(&MachineStateJoltage::initial(self), &mut HashMap::new())
    }

    fn fewest_presses_joltage_state(
        &self,
        state: &MachineStateJoltage,
        memoized: &mut HashMap<Vec<u16>, Option<usize>>,
    ) -> Option<usize> {
        if state.is_done() {
            Some(0)
        } else if let Some(&result) = memoized.get(&state.counters) {
            result
        } else {
            let result = (0..(1 << self.buttons.len()))
                .flat_map(|button_mask| state.press_buttons(self, button_mask))
                .filter_map(|state| {
                    state.half().and_then(|half| {
                        self.fewest_presses_joltage_state(&half, memoized)
                            .map(|half_presses| state.presses + 2 * half_presses)
                    })
                })
                .min();
            memoized.insert(state.counters.clone(), result);
            result
        }
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
        let buttons = parts.map(|p| p.parse().unwrap()).collect::<Vec<_>>();
        Ok(Machine {
            target_lights,
            buttons,
            joltage_requirements,
        })
    }
}

struct MachineStateLights {
    lights: u16,
    presses: u16,
}

impl MachineStateLights {
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

#[derive(Clone)]
struct MachineStateJoltage {
    presses: usize,
    counters: Vec<u16>,
}

impl MachineStateJoltage {
    fn new(counters: Vec<u16>, presses: usize) -> Self {
        Self { presses, counters }
    }

    fn initial(machine: &Machine) -> Self {
        Self::new(machine.joltage_requirements.clone(), 0)
    }

    fn half(&self) -> Option<Self> {
        if self.counters.iter().all(|c| c.is_even()) {
            let counters = self.counters.iter().map(|c| c / 2).collect();
            Some(Self::new(counters, 0))
        } else {
            None
        }
    }

    fn is_done(&self) -> bool {
        self.counters.iter().all(|c| *c == 0)
    }

    fn press_button(self, button: &Button) -> Option<Self> {
        if button.values.iter().all(|b| self.counters[*b as usize] > 0) {
            let counters = button
                .values
                .iter()
                .fold(self.counters.clone(), |mut counters, b| {
                    counters[*b as usize] -= 1;
                    counters
                });
            Some(Self::new(counters, self.presses + 1))
        } else {
            None
        }
    }

    fn press_buttons(&self, machine: &Machine, button_mask: u16) -> Option<Self> {
        machine
            .buttons
            .iter()
            .enumerate()
            .filter_map(|(i, button)| {
                if ((1 << i) & button_mask) != 0 {
                    Some(button)
                } else {
                    None
                }
            })
            .try_fold(self.clone(), |state, button| state.press_button(button))
    }
}

fn part1(input: Lines) -> String {
    input
        .map(|line| line.parse::<Machine>().expect("valid machine"))
        .map(|machine| machine.fewest_presses_lights())
        .sum::<usize>()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .map(|line| line.parse::<Machine>().expect("valid machine"))
        .map(|machine| machine.fewest_presses_joltage().unwrap())
        .sum::<usize>()
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
        verify!(part1, input, "7");
        verify!(part2, input, "33");
    }

    #[test]
    fn test_fewest_presses_lights() {
        let input = include_str!("example.txt");
        let fewest: Vec<_> = input
            .lines()
            .map(|line| line.parse::<Machine>().expect("valid machine"))
            .map(|machine| machine.fewest_presses_lights())
            .collect();
        assert_eq!(fewest, [2, 3, 2]);
    }
}
