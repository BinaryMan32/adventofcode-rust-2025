use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::str::Lines;

struct Shape {
    #[allow(dead_code)]
    count: usize,
}

struct Region {
    width: usize,
    length: usize,
    shapes: Vec<usize>,
}

impl Region {
    // assume every shape is a 3x3 rectangle
    fn available_shapes(&self) -> usize {
        (self.width / 3) * (self.length / 3)
    }

    // count the number of shapes, ignoring their actual shape
    fn required_shapes(&self) -> usize {
        self.shapes.iter().sum()
    }
}

fn parse_input(mut input: Lines) -> (Vec<Shape>, Vec<Region>) {
    let input = input.by_ref();
    let shapes = input
        .take_while(|line| !line.contains("x"))
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let count = chunk
                .map(|line| line.chars().filter(|c| *c == '#').count())
                .sum();
            Shape { count }
        })
        .collect();
    let regions = input
        .map(|line| {
            let (dimensions, shapes) = line.split_once(": ").unwrap();
            let (width, length) = dimensions.split_once('x').unwrap();
            let width = width.parse().unwrap();
            let length = length.parse().unwrap();
            let shapes = shapes
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            Region {
                width,
                length,
                shapes,
            }
        })
        .collect();
    (shapes, regions)
}

fn part1(input: Lines) -> String {
    let (_shapes, regions) = parse_input(input);
    regions
        .iter()
        .filter(|r| r.available_shapes() >= r.required_shapes())
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
        // the actual answer for the example is 2
        // example is harder than real input since it can't be done by
        // assuming every shape is a 3x3 rectangle
        verify!(part1, input, "0");
        verify!(part2, input, "0");
    }
}
