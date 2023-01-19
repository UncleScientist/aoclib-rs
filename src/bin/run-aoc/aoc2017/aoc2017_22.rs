use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_22 {
    infected_nodes: Vec<(i64, i64)>,
    width: i64,
    height: i64,
}

impl Aoc2017_22 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_22 {
    fn name(&self) -> (usize, usize) {
        (2017, 22)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-22.txt");

        self.width = lines[0].len() as i64;
        self.height = lines.len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.infected_nodes.push((row as i64, col as i64));
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid = Grid::from_iter(self.infected_nodes.clone(), self.width, self.height);
        for _ in 0..10_000 {
            grid.step();
        }
        crate::output(grid.infection_count)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug)]
struct Grid {
    nodes: HashSet<(i64, i64)>,
    pos: (i64, i64),
    dir: usize,
    infection_count: usize,
}

impl Grid {
    const DIR: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn _test_new() -> Self {
        let testdata: [(i64, i64); 2] = [(0, 2), (1, 0)];
        Self {
            nodes: HashSet::from_iter(testdata),
            dir: 0,
            pos: (1, 1),
            infection_count: 0,
        }
    }

    fn from_iter<I: IntoIterator<Item = (i64, i64)>>(iter: I, width: i64, height: i64) -> Self {
        Self {
            nodes: HashSet::from_iter(iter),
            pos: (width / 2, height / 2),
            dir: 0,
            infection_count: 0,
        }
    }

    fn step(&mut self) {
        if self.nodes.contains(&self.pos) {
            self.dir = (self.dir + 1) % Self::DIR.len();
            self.nodes.remove(&self.pos);
        } else {
            self.dir = (self.dir + Self::DIR.len() - 1) % Self::DIR.len();
            self.nodes.insert(self.pos);
            self.infection_count += 1;
        }
        self.pos.0 += Self::DIR[self.dir].0;
        self.pos.1 += Self::DIR[self.dir].1;
    }
}
