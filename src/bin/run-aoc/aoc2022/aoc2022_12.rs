use std::collections::{HashSet, VecDeque};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_12 {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Aoc2022_12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_12 {
    fn name(&self) -> (usize, usize) {
        (2022, 12)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-12.txt");

        for (row, line) in lines.iter().enumerate() {
            let mut gridline = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
            if let Some(start_point) = gridline.iter().position(|&p| p == b'S') {
                self.start = (row, start_point);
                gridline[start_point] = b'a';
            }
            if let Some(end_point) = gridline.iter().position(|&p| p == b'E') {
                self.end = (row, end_point);
                gridline[end_point] = b'z';
            }
            self.grid.push(gridline);
        }

        self.width = self.grid[0].len();
        self.height = self.grid.len();
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.bfs(self.start, false).unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.bfs(self.end, true).unwrap())
    }
}

impl Aoc2022_12 {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn get_surrounding_points(
        &self,
        pos: (usize, usize),
        f: &dyn Fn((usize, usize), u8) -> bool,
    ) -> Vec<(usize, usize)> {
        let ipos = (pos.0 as i32, pos.1 as i32);
        let width = self.width as i32;
        let height = self.height as i32;
        let cur_elevation = self.grid[pos.0][pos.1];

        Self::DIR
            .iter()
            .map(|d| (ipos.0 + d.0, ipos.1 + d.1))
            .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < height && pos.1 < width)
            .map(|pos| (pos.0 as usize, pos.1 as usize))
            .filter(|pos| f(*pos, cur_elevation))
            .collect()
    }

    fn bfs(&self, start_point: (usize, usize), backwards: bool) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start_point);
        queue.push_back((start_point, 0));

        while let Some(loc) = queue.pop_front() {
            let valid = if backwards {
                self.get_surrounding_points(loc.0, &|pos: (usize, usize), elevation: u8| {
                    self.grid[pos.0][pos.1] >= elevation - 1
                })
            } else {
                self.get_surrounding_points(loc.0, &|pos: (usize, usize), elevation: u8| {
                    self.grid[pos.0][pos.1] <= elevation + 1
                })
            };

            for v in valid {
                if !visited.insert(v) {
                    continue;
                }

                if (backwards && self.grid[v.0][v.1] == b'a') || (!backwards && v == self.end) {
                    return Some(loc.1 + 1);
                }

                queue.push_back((v, loc.1 + 1));
            }
        }

        None
    }
}
