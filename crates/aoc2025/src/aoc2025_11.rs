use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_11 {
    reactor: HashMap<String, Vec<String>>,
}

impl Aoc2025_11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_11 {
    fn name(&self) -> (usize, usize) {
        (2025, 11)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-11.txt");
        for line in lines {
            let (reactor, nodes) = line.split_once(": ").unwrap();
            let nodes = nodes.split(' ').map(|n| n.to_string()).collect();
            self.reactor.insert(reactor.into(), nodes);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut cache = Cache::default();
        aoclib::output(cache.find(&self.reactor, "you", "out"))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Default)]
struct Cache {
    cache: HashMap<String, usize>,
}

impl Cache {
    fn find(&mut self, tree: &HashMap<String, Vec<String>>, start: &str, end: &str) -> usize {
        if start == end {
            return 1;
        }

        if let Some(answer) = self.cache.get(start) {
            return *answer;
        }

        let count = if let Some(nodelist) = tree.get(start) {
            nodelist
                .iter()
                .map(|node| self.find(tree, node.as_str(), end))
                .sum()
        } else {
            0
        };

        self.cache.insert(start.to_string(), count);
        count
    }
}
