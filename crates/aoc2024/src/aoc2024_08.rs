use std::collections::{HashMap, HashSet};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_08 {
    grid: HashMap<char, Vec<(i64, i64)>>,
    rows: i64,
    cols: i64,
}

impl Aoc2024_08 {
    pub fn new() -> Self {
        Self::default()
    }

    fn in_bounds(&self, loc: &(i64, i64)) -> bool {
        loc.0 >= 0 && loc.1 >= 0 && loc.0 < self.rows && loc.1 < self.cols
    }
}

impl Runner for Aoc2024_08 {
    fn name(&self) -> (usize, usize) {
        (2024, 8)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-08.txt");

        self.rows = lines.len() as i64;
        self.cols = lines[0].len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != '.' {
                    let entry = self.grid.entry(ch).or_default();
                    entry.push((row as i64, col as i64));
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut antinodes = HashSet::new();
        for list in self.grid.values() {
            for i in 0..list.len() - 1 {
                for j in i + 1..list.len() {
                    let rise = list[i].0 - list[j].0;
                    let run = list[i].1 - list[j].1;
                    let l1 = (list[i].0 + rise, list[i].1 + run);
                    let l2 = (list[j].0 - rise, list[j].1 - run);

                    if self.in_bounds(&l1) {
                        antinodes.insert(l1);
                    }
                    if self.in_bounds(&l2) {
                        antinodes.insert(l2);
                    }
                }
            }
        }
        aoclib::output(antinodes.len())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
