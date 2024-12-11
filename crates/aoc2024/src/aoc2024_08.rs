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

    fn get_freq(&self, mut loc: (i64, i64), rise: i64, run: i64, all: bool) -> Vec<(i64, i64)> {
        let mut result = Vec::new();
        loc.0 += rise;
        loc.1 += run;

        while self.in_bounds(&loc) {
            result.push(loc);
            loc.0 += rise;
            loc.1 += run;
            if !all {
                break;
            }
        }
        result
    }

    fn antinodes(&self, list: &Vec<(i64, i64)>, all: bool) -> Vec<(i64, i64)> {
        let mut result = Vec::new();
        if all {
            // need to include antenna locations as well
            result.extend(list);
        }
        for i in 0..list.len() - 1 {
            for j in i + 1..list.len() {
                let rise = list[i].0 - list[j].0;
                let run = list[i].1 - list[j].1;
                result.extend(self.get_freq(list[i], rise, run, all));
                result.extend(self.get_freq(list[j], -rise, -run, all));
            }
        }
        result
    }
}

impl Runner for Aoc2024_08 {
    fn name(&self) -> (usize, usize) {
        (2024, 8)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-08.txt");
        let _lines = aoclib::read_lines("test08.txt");

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
        for nodes in self.grid.values().map(|list| self.antinodes(list, false)) {
            antinodes.extend(nodes);
        }
        aoclib::output(antinodes.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut antinodes = HashSet::new();
        for nodes in self.grid.values().map(|list| self.antinodes(list, true)) {
            antinodes.extend(nodes);
        }
        aoclib::output(antinodes.len())
    }
}
