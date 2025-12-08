use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_04 {
    rolls: HashSet<(isize, isize)>,
    rows: isize,
    cols: isize,
}

impl Aoc2025_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_04 {
    fn name(&self) -> (usize, usize) {
        (2025, 4)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-04.txt");
        // let lines = aoclib::read_lines("input/test.txt");
        self.rows = lines.len() as isize;
        self.cols = lines[0].len() as isize;
        for (row, line) in lines.iter().enumerate() {
            let row = row as isize;
            for (col, ch) in line.chars().enumerate() {
                if ch == '@' {
                    self.rolls.insert((row, col as isize));
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.find_accessible().len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total_removed = 0;
        loop {
            let acc = self.find_accessible();
            if acc.is_empty() {
                break;
            }

            total_removed += acc.len();
            for roll in &acc {
                self.rolls.remove(roll);
            }
        }
        aoclib::output(total_removed)
    }
}

impl Aoc2025_04 {
    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn find_accessible(&self) -> Vec<(isize, isize)> {
        let mut result = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if !self.rolls.contains(&(row, col)) {
                    continue;
                }
                if Self::DIRS
                    .iter()
                    .map(|point| (row + point.0, col + point.1))
                    .filter(|point| self.rolls.contains(point))
                    .count()
                    < 4
                {
                    result.push((row, col));
                }
            }
        }

        result
    }
}
