use std::{convert::Infallible, str::FromStr};

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2025_09 {
    tiles: Vec<Point>,
}

impl Aoc2025_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_09 {
    fn name(&self) -> (usize, usize) {
        (2025, 9)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-09.txt");
        self.tiles = lines.parse_lines();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut max_area = 0;
        for i in 0..self.tiles.len() - 1 {
            for j in i + 1..self.tiles.len() {
                max_area = max_area.max(self.tiles[i].area_between(&self.tiles[j]));
            }
        }

        aoclib::output(max_area)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct Point(usize, usize);

impl Point {
    fn area_between(&self, other: &Point) -> usize {
        (1 + self.0.abs_diff(other.0)) * (1 + self.1.abs_diff(other.1))
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').unwrap();
        Ok(Point(left.parse().unwrap(), right.parse().unwrap()))
    }
}
