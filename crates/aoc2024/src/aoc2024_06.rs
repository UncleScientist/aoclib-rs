use std::collections::HashSet;

use aoclib::{Direction, Position64, Runner, Size64};

#[derive(Default)]
pub struct Aoc2024_06 {
    lab: HashSet<Position64>,
    visited: HashSet<Position64>,
    size: Size64,
    guard: Position64,
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

        self.size = Size64(lines.len() as i64, lines[0].len() as i64);

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.lab.insert(Position64(row as i64, col as i64));
                } else if ch == '^' {
                    self.guard = Position64(row as i64, col as i64);
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut guard = self.guard;
        let mut cur_dir = Direction::UP;
        while guard.in_maze(&self.size) {
            self.visited.insert(guard);
            let new_loc = guard + cur_dir;
            if self.lab.contains(&new_loc) {
                cur_dir = cur_dir.right();
            } else {
                guard = new_loc;
            }
        }
        aoclib::output(self.visited.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        for pos in self.visited.iter() {
            if *pos == self.guard {
                continue;
            }

            let mut loop_detector = HashSet::new();
            let mut guard = self.guard;
            let mut cur_dir = Direction::UP;
            while guard.in_maze(&self.size) {
                if loop_detector.insert((guard, cur_dir)) {
                    let new_loc = guard + cur_dir;
                    if new_loc == *pos || self.lab.contains(&new_loc) {
                        cur_dir = cur_dir.right();
                    } else {
                        guard = new_loc;
                    }
                } else {
                    total += 1;
                    break;
                }
            }
        }
        aoclib::output(total)
    }
}
