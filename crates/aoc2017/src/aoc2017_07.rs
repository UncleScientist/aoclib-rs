use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_07 {
    tree: HashMap<String, Node>,
    root_name: String,
}

impl Aoc2017_07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_07 {
    fn name(&self) -> (usize, usize) {
        (2017, 7)
    }

    fn parse(&mut self) {
        let mut lhs = HashSet::new();
        let mut rhs = HashSet::new();

        for line in aoclib::read_lines("input/2017-07.txt") {
            let words = line.split(" -> ").collect::<Vec<&str>>();
            let (left, wstr) = words[0].split_once(' ').unwrap();
            let weight = wstr[1..wstr.len() - 1].parse::<usize>().unwrap();
            let mut node = Node {
                weight,
                child: Vec::new(),
            };

            lhs.insert(left.to_string());

            if words.len() > 1 {
                for right in words[1].split(", ") {
                    node.child.push(right.to_string());
                    rhs.insert(right.to_string());
                }
            }
            self.tree.insert(left.to_string(), node);
        }
        self.root_name = lhs.difference(&rhs).take(1).next().unwrap().clone();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(&self.root_name)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut ptr = self.tree.get(&self.root_name).unwrap();
        let mut diff = 0i32;

        let bad_node = loop {
            let mut weights = ptr
                .child
                .iter()
                .map(|c| (c, get_weight(&self.tree, c)))
                .collect::<Vec<(_, _)>>();
            weights.sort_by(|a, b| a.1.cmp(&b.1));
            let last = weights.len() - 1;
            if weights[0].1 == weights[last].1 {
                // it's balanced from this point up
                break ptr;
            } else if weights[0].1 == weights[1].1 {
                diff = (weights[0].1 as i32) - (weights[last].1 as i32);
                ptr = self.tree.get(weights[last].0).unwrap();
            } else {
                diff = (weights[1].1 as i32) - (weights[0].1 as i32);
                ptr = self.tree.get(weights[0].0).unwrap();
            }
        };

        aoclib::output((bad_node.weight as i32) + diff)
    }
}

#[derive(Debug)]
struct Node {
    weight: usize,
    child: Vec<String>,
}

fn get_weight(root: &HashMap<String, Node>, name: &str) -> usize {
    let node = root.get(name).unwrap();
    node.weight
        + node
            .child
            .iter()
            .map(|n| get_weight(root, n))
            .sum::<usize>()
}
