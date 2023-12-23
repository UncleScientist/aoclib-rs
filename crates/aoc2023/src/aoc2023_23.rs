use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_23 {
    maze: HashMap<(i64, i64), Tile>,
    width: i64,
    height: i64,
    start: (i64, i64),
    end: (i64, i64),
}

impl Aoc2023_23 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_23 {
    fn name(&self) -> (usize, usize) {
        (2023, 23)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-23.txt");
        self.height = lines.len() as i64;
        self.width = lines[0].len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let row = row as i64;
                let col = col as i64;
                match ch {
                    '#' => {}
                    '.' => {
                        self.maze.insert((row, col), Tile::Space);
                    }
                    '>' => {
                        self.maze.insert((row, col), Tile::Slope((0, 1)));
                    }
                    'v' => {
                        self.maze.insert((row, col), Tile::Slope((1, 0)));
                    }
                    _ => panic!("invalid character"),
                }
            }
        }

        for col in 0..self.width {
            if self.maze.contains_key(&(0, col)) {
                self.start = (0, col);
            } else if self.maze.contains_key(&(self.height - 1, col)) {
                self.end = (self.height - 1, col);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut stack = vec![(self.start, (1, 0), 0)];
        let mut max_dist = 0;
        while let Some((curpos, curdir, dist)) = stack.pop() {
            if curpos == self.end {
                max_dist = max_dist.max(dist);
                continue;
            }
            for dir in &DIRS {
                if (-dir.0, -dir.1) == curdir {
                    continue;
                }
                let nextpos = (curpos.0 + dir.0, curpos.1 + dir.1);
                if let Some(tile) = self.maze.get(&nextpos) {
                    match tile {
                        Tile::Space => {
                            stack.push((nextpos, *dir, dist + 1));
                        }
                        Tile::Slope(slope) => {
                            if slope == dir {
                                stack.push((nextpos, *dir, dist + 1));
                            }
                        }
                    }
                }
            }
        }
        aoclib::output(max_dist)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

const DIRS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
enum Tile {
    Space,
    Slope((i64, i64)),
}
