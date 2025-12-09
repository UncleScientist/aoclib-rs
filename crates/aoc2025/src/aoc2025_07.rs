use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_07 {
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
        aoclib::output("unsolved")
    }
}
