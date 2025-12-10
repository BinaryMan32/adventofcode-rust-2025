use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::ops::RangeInclusive;
use std::{
    iter::repeat_n,
    ops::{Add, Sub},
    str::{FromStr, Lines},
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn rectangle_area_with(&self, other: &Pos) -> i64 {
        let size = (*other - *self).abs() + 1;
        size.x * size.y
    }

    fn abs(&self) -> Pos {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i64> for Pos {
    type Output = Pos;

    fn add(self, rhs: i64) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<i64> for Pos {
    type Output = Pos;

    fn sub(self, rhs: i64) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or(format!("Pos didn't contain at least one ',' \"{}\"", s))?;
        Ok(Pos {
            x: x.parse()
                .map_err(|e| format!("Unable to parse x coord \"{}\" due to: {}", x, e))?,
            y: y.parse()
                .map_err(|e| format!("Unable to parse y coord \"{}\" due to: {}", x, e))?,
        })
    }
}

fn part1(input: Lines) -> String {
    let points = input
        .into_iter()
        .map(|line| line.parse::<Pos>().unwrap())
        .collect_vec();
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.rectangle_area_with(b))
        .max()
        .expect("at least one element")
        .to_string()
}

fn get_bounds(points: &[Pos]) -> (Pos, Pos) {
    let bounds_x = points.iter().map(|p| p.x).minmax().into_option().unwrap();
    let bounds_y = points.iter().map(|p| p.y).minmax().into_option().unwrap();
    (
        Pos {
            x: bounds_x.0,
            y: bounds_y.0,
        },
        Pos {
            x: bounds_x.1,
            y: bounds_y.1,
        },
    )
}

#[derive(Clone, Debug)]
struct Edge {
    x: i64,
    dy: i64,
}

impl Edge {
    fn from_points(a: &Pos, b: &Pos) -> Option<(RangeInclusive<i64>, Self)> {
        if a.x == b.x {
            Some((
                a.y.min(b.y)..=a.y.max(b.y),
                Self {
                    x: a.x,
                    dy: (b.y - a.y).signum(),
                },
            ))
        } else {
            None
        }
    }
}

struct Rasterizer {
    min_y: i64,
    rows: Vec<Vec<Edge>>,
}

impl Rasterizer {
    fn new(min_y: i64, max_y: i64) -> Self {
        let size = (max_y - min_y + 1) as usize;
        let rows = repeat_n(Vec::new(), size).collect_vec();
        Self { min_y, rows }
    }

    fn add_polygon(&mut self, points: &[Pos]) {
        for (a, b) in points.iter().zip(points[1..].iter()) {
            self.add_edge(a, b);
        }
        if points.len() >= 2 {
            self.add_edge(points.last().unwrap(), points.first().unwrap());
        }
    }

    fn add_edge(&mut self, a: &Pos, b: &Pos) {
        if let Some((range, edge)) = Edge::from_points(a, b) {
            for y in range {
                self.rows[(y - self.min_y) as usize].push(edge.clone());
            }
        }
    }

    fn print(&self) {
        for (y, row) in self.rows.iter().enumerate() {
            println!("rasterizer[{}] = {:?}", y as i64 + self.min_y, row);
        }
    }

    fn render(self) -> Rendered {
        let rows = self
            .rows
            .into_iter()
            .map(RenderedRow::from_edges)
            .collect_vec();
        Rendered {
            min_y: self.min_y,
            rows,
        }
    }
}

#[derive(Debug)]
struct RenderedRow {
    intervals: Vec<RangeInclusive<i64>>,
}

impl RenderedRow {
    fn from_edges(mut edges: Vec<Edge>) -> Self {
        edges.sort_by_key(|e| e.x);
        let mut simplified = Vec::new();
        if let Some(first) = edges.first() {
            simplified.push(first.x);
        }
        let mut is_inside = true;
        for (prev, cur) in edges.into_iter().tuple_windows() {
            if prev.dy != cur.dy {
                simplified.push(cur.x);
                is_inside = !is_inside;
            } else if !is_inside {
                *simplified.last_mut().unwrap() = cur.x
            }
        }
        let intervals = simplified
            .into_iter()
            .tuples()
            .map(|(a, b)| a..=b)
            .collect_vec();
        Self { intervals }
    }

    fn contains(&self, test: &RangeInclusive<i64>) -> bool {
        self.intervals
            .iter()
            .any(|interval| test.start() >= interval.start() && test.end() <= interval.end())
    }
}

struct Rendered {
    min_y: i64,
    rows: Vec<RenderedRow>,
}

impl Rendered {
    fn is_rect_valid(&self, a: &Pos, b: &Pos) -> bool {
        let min_y = (a.y.min(b.y) - self.min_y) as usize;
        let max_y = (a.y.max(b.y) - self.min_y) as usize;
        let range_x = a.x.min(b.x)..=a.x.max(b.x);
        (min_y..=max_y).all(|y| self.rows[y].contains(&range_x))
    }

    fn print(&self) {
        for (y, row) in self.rows.iter().enumerate() {
            println!("rendered[{}] = {:?}", y as i64 + self.min_y, row);
        }
    }
}

fn part2(input: Lines) -> String {
    let points = input
        .into_iter()
        .map(|line| line.parse::<Pos>().unwrap())
        .collect_vec();
    let (min, max) = get_bounds(&points);

    let mut rasterizer = Rasterizer::new(min.y, max.y);
    rasterizer.add_polygon(&points);
    if points.len() < 100 {
        rasterizer.print();
    }
    let rendered = rasterizer.render();
    if points.len() < 100 {
        rendered.print();
    }
    points
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            if rendered.is_rect_valid(a, b) {
                Some(a.rectangle_area_with(b))
            } else {
                None
            }
        })
        .max()
        .expect("at least one element")
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
        verify!(part1, input, "50");
        verify!(part2, input, "24");
    }
}
