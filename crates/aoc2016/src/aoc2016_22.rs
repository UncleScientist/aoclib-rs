use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2016_22 {
    nodes: HashMap<(usize, usize), GridNode>,
    empty: (usize, usize),
    largest: usize,
    width: usize,
    height: usize,
}

impl Aoc2016_22 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

struct GridNode {
    used: usize,
    avail: usize,
}

impl Runner for Aoc2016_22 {
    fn name(&self) -> (usize, usize) {
        (2016, 22)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2016-22.txt");

        for l in lines.iter().skip(2) {
            let col = l.split_whitespace().collect::<Vec<&str>>();
            let loc = col[0].split('-').skip(1).collect::<Vec<&str>>();
            let loc = (
                loc[0][1..].parse::<usize>().unwrap(),
                loc[1][1..].parse::<usize>().unwrap(),
            );
            self.width = self.width.max(loc.0);
            self.height = self.height.max(loc.1);
            let used = col[2][0..col[2].len() - 1].parse::<usize>().unwrap();
            if used == 0 {
                self.empty = loc;
            }
            let avail = col[3][0..col[3].len() - 1].parse::<usize>().unwrap();
            self.largest = self.largest.max(avail);
            self.nodes.insert(loc, GridNode { used, avail });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let count = self
            .nodes
            .iter()
            .filter(|(_, a_node)| a_node.used != 0)
            .fold(0, |sum, a| {
                sum + self
                    .nodes
                    .iter()
                    .filter(|b| a.0 != b.0 && a.1.used <= b.1.avail)
                    .count()
            });
        aoclib::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.empty.0 + self.empty.1 + 6 * (self.width - 1) + 1)
    }
}
