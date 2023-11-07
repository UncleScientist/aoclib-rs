use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_07 {
    graph: BTreeMap<char, Node>,
    possible: BTreeSet<char>,
}

impl Aoc2018_07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_07 {
    fn name(&self) -> (usize, usize) {
        (2018, 7)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2018-07.txt");

        for l in lines {
            let words = l.split_whitespace().collect::<Vec<_>>();
            let node = words[1].chars().next().unwrap();
            let prereq = words[7].chars().next().unwrap();
            self.possible.insert(node);
            self.possible.insert(prereq);

            let e = self.graph.entry(node).or_insert(Node::default());
            e.prereq.insert(prereq);

            let e = self.graph.entry(prereq).or_insert(Node::default());
            e.count += 1;
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut completed = String::new();
        for node in self.graph.values() {
            for ch in node.prereq.iter() {
                self.possible.remove(ch);
            }
        }

        while let Some(item) = self.possible.pop_first() {
            completed.push(item);
            let prereq = self.graph.get(&item).unwrap().clone();
            for finished in prereq.prereq {
                let node = self.graph.get_mut(&finished).unwrap();
                node.count -= 1;
                if node.count == 0 {
                    self.possible.insert(finished);
                }
            }
        }

        crate::output(completed)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Default, Debug, Clone)]
struct Node {
    prereq: HashSet<char>,
    count: i64,
}
