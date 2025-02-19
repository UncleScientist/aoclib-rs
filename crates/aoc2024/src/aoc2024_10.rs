use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_10 {
    grid: Vec<Vec<u8>>,
    rows: i64,
    cols: i64,
    zeroes: Vec<(usize, usize)>,
}

impl Aoc2024_10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn count_paths(&self, ignore_visited: bool) -> usize {
        self.zeroes
            .iter()
            .map(|(r, c)| self.count_paths_from_location(*r as i64, *c as i64, ignore_visited))
            .sum::<usize>()
    }

    fn count_paths_from_location(&self, row: i64, col: i64, ignore_visited: bool) -> usize {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        let mut found = 0;
        stack.push((row, col, 0));
        while let Some((r, c, level)) = stack.pop() {
            if ignore_visited || visited.insert((r, c)) {
                if level == 9 {
                    found += 1;
                    continue;
                }
                for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let new_row = r + dir.0;
                    let new_col = c + dir.1;
                    if new_row < 0 || new_col < 0 || new_row >= self.rows || new_col >= self.cols {
                        continue;
                    }
                    if self.grid[new_row as usize][new_col as usize] == level + 1 {
                        stack.push((new_row, new_col, level + 1));
                    }
                }
            }
        }

        found
    }
}

impl Runner for Aoc2024_10 {
    fn name(&self) -> (usize, usize) {
        (2024, 10)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-10.txt");

        self.rows = lines.len() as i64;
        self.cols = lines[0].len() as i64;

        for line in lines {
            self.grid
                .push(line.chars().map(|ch| ch as u8 - b'0').collect())
        }

        self.zeroes = (0..self.rows)
            .flat_map(|row| {
                (0..self.cols)
                    .map(move |col| (row as usize, col as usize))
                    .filter(|(r, c)| self.grid[*r][*c] == 0)
            })
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.count_paths(false))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.count_paths(true))
    }
}
