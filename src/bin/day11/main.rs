use advent_of_code::{Named, Runner, create_runner, named};
use itertools::Itertools;
use std::{
    collections::HashMap,
    iter::once,
    str::{FromStr, Lines},
};

#[derive(Debug, PartialEq)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl FromStr for Device {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, outputs) = s.split_once(':').ok_or(": after name")?;
        let name = name.to_owned();
        let outputs = outputs.split_whitespace().map(String::from).collect();
        Ok(Self { name, outputs })
    }
}

struct Node {
    outputs: Vec<usize>,
}

impl Node {
    fn from_device<LookupFn: Fn(&str) -> usize>(device: &Device, lookup: LookupFn) -> Self {
        let outputs = device.outputs.iter().map(|o| lookup(o)).collect();
        Self { outputs }
    }
}

struct Graph {
    nodes: Vec<Node>,
    names: HashMap<String, usize>,
}

impl Graph {
    fn from_devices(devices: &[Device]) -> Self {
        let out_device = Device {
            name: "out".to_string(),
            outputs: Vec::new(),
        };
        let names: HashMap<_, _> = devices
            .iter()
            .chain(once(&out_device))
            .enumerate()
            .map(|(i, d)| (d.name.clone(), i))
            .collect();
        let nodes = devices
            .iter()
            .chain(once(&out_device))
            .map(|d| Node::from_device(d, |n| *names.get(n).unwrap()))
            .collect();
        Self { nodes, names }
    }

    fn count_paths_name(&self, from: &str, to: &str) -> usize {
        let from = *self.names.get(from).unwrap();
        let to = *self.names.get(to).unwrap();
        let mut node_paths = HashMap::new();
        self.count_paths_index(from, to, &mut node_paths)
    }

    fn count_paths_index(
        &self,
        from: usize,
        to: usize,
        node_paths: &mut HashMap<usize, usize>,
    ) -> usize {
        if from == to {
            1
        } else if let Some(&num_paths) = node_paths.get(&from) {
            num_paths
        } else {
            let num_paths = self.nodes[from]
                .outputs
                .iter()
                .map(|&o| self.count_paths_index(o, to, node_paths))
                .sum();
            node_paths.entry(from).insert_entry(num_paths);
            num_paths
        }
    }
}

fn part1(input: Lines) -> String {
    let devices = input
        .map(|line| line.parse::<Device>().unwrap())
        .collect_vec();
    Graph::from_devices(&devices)
        .count_paths_name("you", "out")
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
        verify!(part1, input, "5");
        verify!(part2, input, "0");
    }

    #[test]
    fn test_machine_parse() {
        assert_eq!(
            "ccc: ddd eee fff".parse::<Device>(),
            Ok(Device {
                name: "ccc".to_string(),
                outputs: ["ddd", "eee", "fff"].map(|s| s.to_string()).to_vec()
            })
        )
    }
}
