use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::{
    collections::VecDeque,
    iter::{repeat_n, successors},
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

    fn signum(&self) -> Pos {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn line_to(&self, other: &Pos) -> impl Iterator<Item = Pos> {
        let inc = (*other - *self).signum();
        if (inc.x != 0) == (inc.y != 0) {
            panic!(
                "line_to() only supports points differing in exactly one dimension self={:?} other={:?} inc={:?}",
                self, other, inc
            );
        }
        successors(Some(*self), move |&p| Some(p + inc).filter(|p| p != other))
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

#[derive(Clone)]
struct Canvas<Data> {
    offset: Pos,
    size: Pos,
    data: Vec<Vec<Data>>,
}

impl<Data: Copy + PartialEq> Canvas<Data> {
    fn new(min: Pos, max: Pos, value: Data) -> Self {
        let offset = min;
        let size = max - min + 1;
        let row = repeat_n(value, size.x as usize).collect_vec();
        let data = repeat_n(row, size.y as usize).collect_vec();
        Self { offset, size, data }
    }

    fn get(&self, pos: &Pos) -> Data {
        let pos = *pos - self.offset;
        self.get_raw(&pos)
    }

    fn get_raw(&self, pos: &Pos) -> Data {
        self.data[pos.y as usize][pos.x as usize]
    }

    fn in_bounds_raw(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.x < self.size.x && pos.y >= 0 && pos.y < self.size.y
    }

    fn get_raw_opt(&self, pos: &Pos) -> Option<Data> {
        if self.in_bounds_raw(pos) {
            Some(self.get_raw(pos))
        } else {
            None
        }
    }

    fn set(&mut self, pos: &Pos, value: Data) {
        let pos = *pos - self.offset;
        self.set_raw(&pos, value);
    }

    fn set_raw(&mut self, pos: &Pos, value: Data) {
        self.data[pos.y as usize][pos.x as usize] = value;
    }

    fn set_line(&mut self, a: &Pos, b: &Pos, value: Data) {
        let a = *a - self.offset;
        let b = *b - self.offset;
        for p in a.line_to(&b) {
            self.set_raw(&p, value);
        }
    }

    fn stroke_polygon(&mut self, points: &[Pos], value: Data) {
        for (a, b) in points.iter().zip(points[1..].iter()) {
            self.set_line(a, b, value);
        }
        if points.len() >= 2 {
            self.set_line(points.last().unwrap(), points.first().unwrap(), value);
        }
    }

    const NEIGHBORS: [Pos; 4] = [
        Pos { x: -1, y: 0 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: 0, y: 1 },
    ];

    fn flood(&mut self, pos: &Pos, new_value: Data) {
        let old_value = self.get(pos);
        let pos = *pos - self.offset;
        let mut queue = VecDeque::from([pos]);
        while let Some(pos) = queue.pop_front() {
            let data = &mut self.data[pos.y as usize][pos.x as usize];
            if *data == old_value {
                *data = new_value;
                for neighbor_offset in Self::NEIGHBORS {
                    let neighbor = pos + neighbor_offset;
                    if self.get_raw_opt(&neighbor) == Some(old_value) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    fn rect_matches(&self, a: &Pos, b: &Pos, f: fn(&Data) -> bool) -> bool {
        let min_x = a.x.min(b.x) as usize;
        let min_y = a.y.min(b.y) as usize;
        let max_x = a.x.max(b.x) as usize;
        let max_y = a.y.max(b.y) as usize;
        (min_y..=max_y).all(|y| self.data[y][min_x..=max_x].iter().all(f))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Inside,
    Point,
    Line,
    Outside,
}

impl Tile {
    fn as_char(&self) -> char {
        match *self {
            Tile::Inside => '+',
            Tile::Point => '#',
            Tile::Line => 'X',
            Tile::Outside => '.',
        }
    }
}

fn print_canvas(canvas: &Canvas<Tile>) {
    for row in canvas.data.iter() {
        for t in row {
            print!("{}", t.as_char());
        }
        println!();
    }
}

fn part2(input: Lines) -> String {
    let points = input
        .into_iter()
        .map(|line| line.parse::<Pos>().unwrap())
        .collect_vec();
    let (min, max) = get_bounds(&points);
    println!("sizeof(Tile)={}", size_of::<Tile>());
    println!("bounds: min={:?} max=={:?}", min, max);
    let mut canvas = Canvas::new(min - 1, max + 1, Tile::Inside);
    canvas.stroke_polygon(&points, Tile::Line);
    points.iter().for_each(|p| canvas.set(p, Tile::Point));
    canvas.flood(&min, Tile::Outside);
    if points.len() < 100 {
        print_canvas(&canvas);
    }
    points
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            if canvas.rect_matches(a, b, |&d| d != Tile::Outside) {
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
