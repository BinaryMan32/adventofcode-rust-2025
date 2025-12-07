use advent_of_code::{Named, Runner, create_runner, named};
use core::{fmt, panic};
use itertools::Itertools;
use std::iter::repeat_n;
use std::ops::Range;
use std::str::Lines;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    Splitter,
    Beam,
}

impl Cell {
    fn as_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Splitter => '^',
            Cell::Beam => '|',
        }
    }
}

struct TachyonManifold {
    cells: Vec<Vec<Cell>>,
    start: (usize, usize),
}

impl fmt::Display for TachyonManifold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let cell = if self.start == (x, y) {
                    'S'
                } else {
                    cell.as_char()
                };
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TachyonManifold {
    fn parse(input: Lines) -> Self {
        let mut start = None;
        let cells = input
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Cell::Empty,
                        '^' => Cell::Splitter,
                        'S' => {
                            start = Some((x, y)); // Placeholder for start position
                            Cell::Empty
                        }
                        _ => panic!("Unknown cell value '{}' at ({}, {})", c, x, y),
                    })
                    .collect()
            })
            .collect();
        let start = start.expect("Start position 'S' not found in input");
        TachyonManifold { cells, start }
    }

    fn start(&mut self) -> Range<usize> {
        let beam_y = self.start.1 + 1;
        let beam_cell = &mut self.cells[beam_y][self.start.0];
        assert_eq!(
            beam_cell,
            &Cell::Empty,
            "Expected Empty cell below the start position"
        );
        *beam_cell = Cell::Beam;
        beam_y..(self.cells.len() - 1)
    }

    fn step(&mut self, y: usize) -> usize {
        let current = self.cells[y].clone();
        let next = &mut (self.cells[y + 1]);
        let mut beam_splits = 0;
        for (x, cell) in current.into_iter().enumerate() {
            if cell == Cell::Beam {
                let next_cell = &mut next[x];
                match *next_cell {
                    Cell::Empty => {
                        *next_cell = Cell::Beam;
                    }
                    Cell::Splitter => {
                        beam_splits += 1;
                        for split_x in [x - 1, x + 1].into_iter() {
                            let split_cell = &mut next[split_x];
                            match *split_cell {
                                Cell::Empty => {
                                    *split_cell = Cell::Beam;
                                }
                                Cell::Splitter => {
                                    panic!(
                                        "Split beam hit another Splitter at ({}, {})",
                                        split_x,
                                        y + 1
                                    );
                                }
                                Cell::Beam => {}
                            }
                        }
                    }
                    Cell::Beam => {}
                };
            }
        }
        beam_splits
    }

    fn run(&mut self) -> usize {
        self.start().map(|y| self.step(y)).sum::<usize>()
    }

    fn count_timelines(&self) -> usize {
        let results_row = repeat_n(None, self.cells[0].len()).collect_vec();
        let mut results = repeat_n(results_row, self.cells.len()).collect_vec();
        self.count_timelines_pos((self.start.0, self.start.1 + 1), &mut results)
    }

    fn count_timelines_pos(
        &self,
        pos: (usize, usize),
        results: &mut Vec<Vec<Option<usize>>>,
    ) -> usize {
        if pos.1 >= self.cells.len() {
            1
        } else {
            match results[pos.1][pos.0] {
                Some(count) => count,
                None => {
                    let count = self.count_timelines_pos_internal(pos, results);
                    results[pos.1][pos.0] = Some(count);
                    count
                }
            }
        }
    }

    fn count_timelines_pos_internal(
        &self,
        pos: (usize, usize),
        results: &mut Vec<Vec<Option<usize>>>,
    ) -> usize {
        match self.cells[pos.1][pos.0] {
            Cell::Empty => self.count_timelines_pos((pos.0, pos.1 + 1), results),
            Cell::Splitter => {
                self.count_timelines_pos((pos.0 - 1, pos.1), results)
                    + self.count_timelines_pos((pos.0 + 1, pos.1), results)
            }
            Cell::Beam => panic!("Beams should not be present"),
        }
    }
}

fn part1(input: Lines) -> String {
    let mut tachyon_manifold = TachyonManifold::parse(input);
    tachyon_manifold.run().to_string()
}

fn part2(input: Lines) -> String {
    TachyonManifold::parse(input).count_timelines().to_string()
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
        verify!(part1, input, "21");
        verify!(part2, input, "40");
    }

    #[test]
    fn test_manifold_layout() {
        let input = include_str!("example.txt");
        let expected = include_str!("expected_part1.txt");
        let mut tachyon_manifold = TachyonManifold::parse(input.lines());
        tachyon_manifold.run();
        assert_eq!(format!("{}", tachyon_manifold), expected);
    }
}
