use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_07 {
    cache: HashMap<(usize, usize), usize>,
    manifold: HashSet<(usize, usize)>,
    rows: usize,
    start: (usize, usize),
}

impl Aoc2025_07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_07 {
    fn name(&self) -> (usize, usize) {
        (2025, 7)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-07.txt");
        // let lines = aoclib::read_lines("test.txt");
        self.rows = lines.len();
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '^' {
                    self.manifold.insert((row, col));
                } else if ch == 'S' {
                    self.start = (row, col);
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut beams = HashSet::from([self.start.1]);
        let mut row = 2;
        let mut splits = 0;

        while row < self.rows {
            let mut next_beams = HashSet::new();

            for beam in &beams {
                if self.manifold.contains(&(row, *beam)) {
                    splits += 1;
                    next_beams.insert(*beam - 1);
                    next_beams.insert(*beam + 1);
                } else {
                    next_beams.insert(*beam);
                }
            }

            row += 2;
            beams = next_beams;
        }

        aoclib::output(splits)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.find_from(self.start))
    }
}

impl Aoc2025_07 {
    fn find_from(&mut self, point: (usize, usize)) -> usize {
        if point.0 >= self.rows {
            return 1;
        }

        if let Some(count) = self.cache.get(&point) {
            return *count;
        }

        let count = if self.manifold.contains(&point) {
            self.find_from((point.0 + 2, point.1 - 1)) + self.find_from((point.0 + 2, point.1 + 1))
        } else {
            self.find_from((point.0 + 2, point.1))
        };

        self.cache.insert(point, count);
        count
    }
}
