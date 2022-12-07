use std::collections::{HashMap, HashSet};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_07 {
    tree: HashMap<String, Node>,
    lhs: HashSet<String>,
    rhs: HashSet<String>,
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
        for line in aoclib::read_lines("input/2017-07.txt") {
            let words = line.split(" -> ").collect::<Vec<&str>>();
            let (left, wstr) = words[0].split_once(' ').unwrap();
            let weight = wstr[1..wstr.len() - 1].parse::<usize>().unwrap();
            let mut node = Node {
                _name: left.to_string(),
                _weight: weight,
                child: Vec::new(),
            };

            self.lhs.insert(left.to_string());

            if words.len() > 1 {
                for right in words[1].split(", ") {
                    node.child.push(right.to_string());
                    self.rhs.insert(right.to_string());
                }
            }
            self.tree.insert(left.to_string(), node);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.lhs.difference(&self.rhs).take(1).next().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug)]
struct Node {
    _name: String,
    _weight: usize,
    child: Vec<String>,
}
