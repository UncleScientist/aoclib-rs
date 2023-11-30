use aoclib::read_to_chars;
use std::collections::HashSet;

use aoclib::Runner;

pub struct Aoc2015_03 {
    data: Vec<char>,
}

impl Aoc2015_03 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Runner for Aoc2015_03 {
    fn parse(&mut self) {
        self.data = read_to_chars("input/2015-03.txt");
    }
    fn name(&self) -> (usize, usize) {
        (2015, 3)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut grid = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        for d in &self.data {
            grid.insert((x, y));
            match d {
                '^' => y -= 1,
                'v' => y += 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => panic!("bad char in input '{d}'"),
            }
        }
        grid.insert((x, y));
        aoclib::output(grid.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut grid = HashSet::new();
        let mut x = [0, 0];
        let mut y = [0, 0];
        let mut which = 0;
        for d in &self.data {
            grid.insert((x[which], y[which]));
            match d {
                '^' => y[which] -= 1,
                'v' => y[which] += 1,
                '>' => x[which] += 1,
                '<' => x[which] -= 1,
                _ => panic!("bad char in input '{d}'"),
            }
            which = 1 - which;
        }
        grid.insert((x[which], y[which]));
        aoclib::output(grid.len())
    }
}
