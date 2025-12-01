use advent_of_code::{Named, Runner, create_runner, named};
use num::traits::Euclid;
use std::str::{FromStr, Lines};

type DialPosition = i16;
type DialDistance = DialPosition;

const DIAL_SIZE: DialPosition = 100;

#[derive(Debug)]
enum DialRotation {
    Left(DialDistance),
    Right(DialDistance),
}

impl FromStr for DialRotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);
        let distance: DialDistance = distance
            .parse()
            .map_err(|_| format!("Failed to parse distance from '{}'", s))?;
        match direction {
            "L" => Ok(DialRotation::Left(distance)),
            "R" => Ok(DialRotation::Right(distance)),
            _ => Err(format!("Unknown direction '{}'", direction)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct DialRotationResult {
    position: DialPosition,
    zero_count: usize,
}

impl DialRotation {
    fn apply(&self, initial_position: DialPosition) -> DialRotationResult {
        match self {
            DialRotation::Left(distance) => {
                let (zero_count, distance) = distance.div_rem_euclid(&DIAL_SIZE);
                let mut position = initial_position - distance;
                let mut zero_count = zero_count as usize;
                if position < 0 {
                    position += DIAL_SIZE;
                    // if we started at zero, we already counted this wrap around last time
                    if initial_position != 0 {
                        zero_count += 1;
                    }
                } else if position == 0 {
                    zero_count += 1;
                }
                DialRotationResult {
                    position,
                    zero_count,
                }
            }
            DialRotation::Right(distance) => {
                let (zero_count, distance) = distance.div_rem_euclid(&DIAL_SIZE);
                let mut position = initial_position + distance;
                let mut zero_count = zero_count as usize;
                if position >= DIAL_SIZE {
                    position -= DIAL_SIZE;
                    zero_count += 1;
                }
                DialRotationResult {
                    position,
                    zero_count,
                }
            }
        }
    }
}

fn part1(input: Lines) -> String {
    input
        .into_iter()
        .map(|line| line.parse::<DialRotation>().unwrap())
        .scan(50, |position, rotation| {
            let result = rotation.apply(*position);
            *position = result.position;
            Some(result.position)
        })
        .filter(|&position| position == 0)
        .count()
        .to_string()
}

fn part2(input: Lines) -> String {
    input
        .into_iter()
        .map(|line| line.parse::<DialRotation>().unwrap())
        .scan(50, |position, rotation| {
            let result = rotation.apply(*position);
            *position = result.position;
            Some(result.zero_count)
        })
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
        verify!(part1, input, "3");
        verify!(part2, input, "6");
    }

    #[test]
    fn rotate_no_zero_left() {
        assert_eq!(
            DialRotation::Left(30).apply(82),
            DialRotationResult {
                position: 52,
                zero_count: 0
            }
        );
    }

    #[test]
    fn rotate_no_zero_right() {
        assert_eq!(
            DialRotation::Right(22).apply(50),
            DialRotationResult {
                position: 72,
                zero_count: 0
            }
        );
    }

    #[test]
    fn rotate_from_zero_left() {
        assert_eq!(
            DialRotation::Left(5).apply(0),
            DialRotationResult {
                position: 95,
                zero_count: 0
            }
        );
    }

    #[test]
    fn rotate_from_zero_right() {
        assert_eq!(
            DialRotation::Right(14).apply(0),
            DialRotationResult {
                position: 14,
                zero_count: 0
            }
        );
    }

    #[test]
    fn rotate_to_zero_left() {
        assert_eq!(
            DialRotation::Left(55).apply(55),
            DialRotationResult {
                position: 0,
                zero_count: 1
            }
        );
    }

    #[test]
    fn rotate_to_zero_right() {
        assert_eq!(
            DialRotation::Right(48).apply(52),
            DialRotationResult {
                position: 0,
                zero_count: 1
            }
        );
    }

    #[test]
    fn rotate_past_zero_left() {
        assert_eq!(
            DialRotation::Left(68).apply(50),
            DialRotationResult {
                position: 82,
                zero_count: 1
            }
        );
    }

    #[test]
    fn rotate_past_zero_right() {
        assert_eq!(
            DialRotation::Right(60).apply(95),
            DialRotationResult {
                position: 55,
                zero_count: 1
            }
        );
    }

    #[test]
    fn rotate_multiple_spins_left() {
        assert_eq!(
            DialRotation::Left(1022).apply(51),
            DialRotationResult {
                position: 29,
                zero_count: 10
            },
            "simple"
        );
        assert_eq!(
            DialRotation::Left(1022).apply(0),
            DialRotationResult {
                position: 78,
                zero_count: 10
            },
            "from zero"
        );
        assert_eq!(
            DialRotation::Left(1022).apply(22),
            DialRotationResult {
                position: 0,
                zero_count: 11
            },
            "to zero"
        );
    }

    #[test]
    fn rotate_multiple_spins_right() {
        assert_eq!(
            DialRotation::Right(1022).apply(51),
            DialRotationResult {
                position: 73,
                zero_count: 10
            },
            "simple"
        );
        assert_eq!(
            DialRotation::Right(1022).apply(0),
            DialRotationResult {
                position: 22,
                zero_count: 10
            },
            "from zero"
        );
        assert_eq!(
            DialRotation::Right(1022).apply(78),
            DialRotationResult {
                position: 0,
                zero_count: 11
            },
            "to zero"
        );
    }
}
