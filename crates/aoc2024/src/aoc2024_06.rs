use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_06 {
    lab: HashSet<(i64, i64)>,
    rows: i64,
    cols: i64,
    guard: (i64, i64),
}

impl Aoc2024_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_06 {
    fn name(&self) -> (usize, usize) {
        (2024, 6)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-06.txt");

        self.rows = lines.len() as i64;
        self.cols = lines[0].len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.lab.insert((row as i64, col as i64));
                } else if ch == '^' {
                    self.guard = (row as i64, col as i64);
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut guard = self.guard;
        let mut cur_dir = 0;
        while guard.0 >= 0 && guard.0 < self.rows && guard.1 >= 0 && guard.1 <= self.cols {
            visited.insert(guard);
            let new_loc = (guard.0 + DIRS[cur_dir].0, guard.1 + DIRS[cur_dir].1);
            if self.lab.contains(&new_loc) {
                cur_dir = (cur_dir + 1) % DIRS.len();
            } else {
                guard = new_loc;
            }
        }
        aoclib::output(visited.len())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

const DIRS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
