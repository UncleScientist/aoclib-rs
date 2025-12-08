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
        let mut accessible = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if !self.rolls.contains(&(row, col)) {
                    continue;
                }
                let mut count: usize = 0;
                for delta in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    count += self.rolls.contains(&(row + delta.0, col + delta.1)) as usize;
                }
                if count < 4 {
                    accessible += 1;
                }
            }
        }
        aoclib::output(accessible)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
