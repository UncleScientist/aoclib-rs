use std::collections::HashMap;

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
        let lines = aoclib::read_lines("input/2022-12.txt"); //"test-input.txt");

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
        let mut to_visit = Vec::new();

        let mut shortest: HashMap<(usize, usize), usize> = HashMap::new();
        shortest.insert(self.start, 0);

        to_visit.extend(self.get_surrounding_points(self.start));
        while let Some(loc) = to_visit.pop() {
            let cur_elevation = self.grid[loc.0][loc.1];
            let points = self.get_surrounding_points(loc);
            let valid = points
                .iter()
                .filter(|pos| self.grid[pos.0][pos.1] + 1 >= cur_elevation)
                .copied()
                .collect::<Vec<(usize, usize)>>();

            let new_path_dist = valid.iter().filter_map(|pos| shortest.get(pos)).min();

            if new_path_dist.is_none() {
                continue;
            }

            let new_path_dist = new_path_dist.unwrap() + 1;

            let cur_path_dist = shortest.entry(loc).or_insert(usize::MAX);
            if *cur_path_dist > new_path_dist {
                *cur_path_dist = new_path_dist;
                to_visit.extend(points.iter());
            }
        }

        crate::output(shortest.get(&self.end).unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

impl Aoc2022_12 {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn get_surrounding_points(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let ipos = (pos.0 as i32, pos.1 as i32);
        let width = self.width as i32;
        let height = self.height as i32;
        Self::DIR
            .iter()
            .map(|d| (ipos.0 + d.0, ipos.1 + d.1))
            .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < height && pos.1 < width)
            .map(|pos| (pos.0 as usize, pos.1 as usize))
            .collect()
    }
}
