use advent_of_code::{Named, Runner, create_runner, named};
use itertools::{Itertools, repeat_n};
use std::str::{FromStr, Lines};

#[derive(Debug)]
struct JunctionBox {
    pos: [i32; 3],
}

impl FromStr for JunctionBox {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s
            .splitn(3, ',')
            .flat_map(|n| n.parse::<i32>())
            .collect_array()
            .ok_or(format!("unable to parse '{}'", s))?;
        Ok(Self { pos })
    }
}

impl JunctionBox {
    fn distance_squared_to(&self, other: &Self) -> u64 {
        self.pos
            .iter()
            .zip(other.pos)
            .map(|(&a, b)| {
                let diff = (a - b).unsigned_abs() as u64;
                diff * diff
            })
            .sum()
    }
}

fn parse_input(input: Lines) -> Vec<JunctionBox> {
    input
        .flat_map(|line| line.parse::<JunctionBox>())
        .collect_vec()
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Connection {
    distance_squared: u64,
    boxes: [usize; 2],
}

fn all_connections_iter(boxes: &[JunctionBox]) -> impl Iterator<Item = Connection> {
    (0..boxes.len()).flat_map(move |i| {
        (0..i).map(move |j| Connection {
            distance_squared: boxes[i].distance_squared_to(&boxes[j]),
            boxes: [i, j],
        })
    })
}

struct Components {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Components {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect_vec();
        let size = repeat_n(1, n).collect_vec();
        Self { parent, size }
    }

    fn find_root(&mut self, i: usize) -> usize {
        let parent = self.parent[i];
        if parent == i {
            i
        } else {
            let root = self.find_root(parent);
            self.parent[i] = root;
            root
        }
    }

    fn union_roots(&mut self, from: usize, into: usize) {
        self.parent[from] = into;
        self.size[into] += self.size[from];
        self.size[from] = 0;
    }

    fn union(&mut self, a: usize, b: usize) {
        let a = self.find_root(a);
        let b = self.find_root(b);
        if a != b {
            if self.size[a] < self.size[b] {
                self.union_roots(a, b);
            } else {
                self.union_roots(b, a);
            }
        }
    }

    fn largest_components(&self, n: usize) -> Vec<usize> {
        self.size.iter().k_largest(n).copied().collect_vec()
    }
}

fn components_add_connection<'a>(
    components: &'a mut Components,
    connection: &Connection,
) -> &'a Components {
    components.union(connection.boxes[0], connection.boxes[1]);
    components
}

fn components_from_connections(connections: &[Connection], n: usize) -> Components {
    let mut components = Components::new(n);
    for connection in connections {
        components_add_connection(&mut components, connection);
    }
    components
}

fn part1(input: Lines) -> String {
    let boxes = parse_input(input);
    let num_connections = if boxes.len() <= 20 { 10 } else { 1000 };
    let connections = all_connections_iter(&boxes)
        .k_smallest(num_connections)
        .collect_vec();
    components_from_connections(&connections, boxes.len())
        .largest_components(3)
        .iter()
        .product::<usize>()
        .to_string()
}

fn part2(input: Lines) -> String {
    let boxes = parse_input(input);
    let mut components = Components::new(boxes.len());
    // just guess that 10,000 is enough since we don't want to test all possible connections
    for connection in all_connections_iter(&boxes).k_smallest(10_000).sorted() {
        components_add_connection(&mut components, &connection);
        if components.largest_components(1)[0] == boxes.len() {
            return connection
                .boxes
                .map(|b| boxes[b].pos[0] as u64)
                .iter()
                .product::<u64>()
                .to_string();
        }
    }
    "".to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let runner: &Runner = create_runner!();
    runner.run(named!(part1), input);
    runner.run(named!(part2), input);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use advent_of_code::verify;

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        verify!(part1, input, "40");
        verify!(part2, input, "25272");
    }

    fn readable_connection(connection: &Connection, boxes: &[JunctionBox]) -> [[i32; 3]; 2] {
        let mut connection = connection.boxes.map(|b| boxes[b].pos);
        connection.sort();
        connection
    }

    #[test]
    fn test_shortest_connections() {
        let input = include_str!("example.txt");
        let boxes = parse_input(input.lines());
        let connections = all_connections_iter(&boxes)
            .k_smallest(4)
            .sorted()
            .map(|c| readable_connection(&c, &boxes))
            .collect_vec();
        assert_eq!(
            connections,
            vec![
                [[162, 817, 812], [425, 690, 689]],
                [[162, 817, 812], [431, 825, 988]],
                [[805, 96, 715], [906, 360, 560]],
                [[425, 690, 689], [431, 825, 988]],
            ]
        );
    }

    fn check_components(actual: &mut Components, expected: Vec<Vec<usize>>) {
        let mut checked: HashMap<usize, Vec<usize>> = HashMap::new();
        for expected_set in expected {
            let expected_root = actual.find_root(expected_set[0]);
            for a in expected_set[1..].iter() {
                let actual_root = actual.find_root(*a);
                assert_eq!(
                    actual_root, expected_root,
                    "{} and {} have different roots {} and {}",
                    a, expected_set[0], actual_root, expected_root
                )
            }
            assert_eq!(actual.size[expected_root], expected_set.len());
            if let Some(conflict) = checked.get(&expected_root) {
                panic!(
                    "{:?} and {:?} should have different roots, both were {}",
                    expected_set, conflict, expected_root
                );
            }
            checked.entry(expected_root).insert_entry(expected_set);
        }
    }

    #[test]
    fn test_union() {
        let mut components = Components::new(4);
        components.union(0, 1);
        check_components(&mut components, vec![vec![0, 1], vec![2], vec![3]]);
        assert_eq!(components.largest_components(4), [2, 1, 1, 0]);

        components.union(2, 0);
        check_components(&mut components, vec![vec![0, 1, 2], vec![3]]);
        assert_eq!(components.largest_components(4), [3, 1, 0, 0]);

        components.union(0, 3);
        check_components(&mut components, vec![vec![0, 1, 2, 3]]);
        assert_eq!(components.largest_components(4), [4, 0, 0, 0]);
    }

    #[test]
    fn test_largest_components() {
        let input = include_str!("example.txt");
        let boxes = parse_input(input.lines());
        assert_eq!(boxes.len(), 20);

        let connections = all_connections_iter(&boxes)
            .k_smallest(10)
            .sorted()
            .collect_vec();
        assert_eq!(connections.len(), 10);
        let mut connections = connections.into_iter();

        let mut components = Components::new(boxes.len());

        assert_eq!(
            components_add_connection(&mut components, &connections.next().unwrap())
                .largest_components(5),
            [2, 1, 1, 1, 1]
        );

        assert_eq!(
            components_add_connection(&mut components, &connections.next().unwrap())
                .largest_components(5),
            [3, 1, 1, 1, 1]
        );

        assert_eq!(
            components_add_connection(&mut components, &connections.next().unwrap())
                .largest_components(5),
            [3, 2, 1, 1, 1]
        );

        assert_eq!(
            components_add_connection(&mut components, &connections.next().unwrap())
                .largest_components(5),
            [3, 2, 1, 1, 1]
        );

        for connection in connections {
            components_add_connection(&mut components, &connection);
        }
        assert_eq!(
            components.largest_components(12),
            [5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0]
        );
    }
}
