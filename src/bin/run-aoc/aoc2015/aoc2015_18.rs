use std::collections::HashSet;

use crate::Runner;

pub struct Aoc2015_18 {
    initial_grid: HashSet<(usize, usize)>,
    grid: HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl Aoc2015_18 {
    pub fn new() -> Self {
        Self {
            initial_grid: HashSet::new(),
            grid: HashSet::new(),
            rows: 0,
            cols: 0,
        }
    }

    fn _dump(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.grid.contains(&(r, c)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn check(pos: usize, lim: usize) -> Vec<usize> {
        if pos == 0 {
            vec![0, 1]
        } else if pos + 1 == lim {
            vec![pos - 1, pos]
        } else {
            vec![pos - 1, pos, pos + 1]
        }
    }

    fn neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for r in Self::check(row, self.rows) {
            for c in Self::check(col, self.cols) {
                if r == row && c == col {
                    continue;
                } else if self.grid.contains(&(r, c)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn corners(&mut self) {
        self.grid.insert((0, 0));
        self.grid.insert((0, self.cols - 1));
        self.grid.insert((self.rows - 1, 0));
        self.grid.insert((self.rows - 1, self.cols - 1));
    }

    fn step(&mut self) {
        let mut next_state = HashSet::new();

        for r in 0..self.rows {
            for c in 0..self.cols {
                let neighbors = self.neighbors(r, c);
                if neighbors == 3 || (self.grid.contains(&(r, c)) && neighbors == 2) {
                    next_state.insert((r, c));
                }
            }
        }

        self.grid = next_state;
    }
}

impl Runner for Aoc2015_18 {
    fn name(&self) -> (usize, usize) {
        (2015, 18)
    }

    fn parse(&mut self) {
        let _test_data = [
            ".#.#.#".to_string(),
            "...##.".to_string(),
            "#....#".to_string(),
            "..#...".to_string(),
            "#.#..#".to_string(),
            "####..".to_string(),
        ];

        let mut row = 0;
        let mut col = 0;
        for line in aoclib::read_lines("input/2015-18.txt") {
            col = 0;
            for c in line.chars() {
                if matches!(c, '#') {
                    self.grid.insert((row, col));
                }
                col += 1;
            }
            row += 1;
        }
        self.rows = row;
        self.cols = col;

        self.initial_grid = self.grid.clone();
    }

    fn part1(&mut self) -> Vec<String> {
        for _ in 0..100 {
            self.step();
        }
        crate::output(self.grid.len())
    }

    fn part2(&mut self) -> Vec<String> {
        self.grid = self.initial_grid.clone();

        for _ in 0..100 {
            self.corners();
            self.step();
        }

        self.corners();
        crate::output(self.grid.len())
    }
}
