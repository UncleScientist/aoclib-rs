use std::{convert::Infallible, str::FromStr};

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2025_08 {
    boxes: Vec<Point>,
}

impl Aoc2025_08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_08 {
    fn name(&self) -> (usize, usize) {
        (2025, 8)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-08.txt");
        // let lines = aoclib::read_lines("test.txt");
        self.boxes = lines.parse_lines();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut distances = Vec::new();
        for i in 0..self.boxes.len() - 1 {
            for j in i + 1..self.boxes.len() {
                distances.push((self.boxes[i].dist_to(&self.boxes[j]), i, j));
            }
        }
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut ds = aoclib::DisjointSet::init_by_size(self.boxes.len());
        for &(_dist, i, j) in distances.iter().take(1000) {
            ds.union(i, j);
        }
        let mut sizes = ds.get_sizes().unwrap();
        sizes.sort_by(|a, b| b.cmp(a));

        aoclib::output(sizes.iter().take(3).product::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut distances = Vec::new();
        for i in 0..self.boxes.len() - 1 {
            for j in i + 1..self.boxes.len() {
                distances.push((self.boxes[i].dist_to(&self.boxes[j]), i, j));
            }
        }
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut ds = aoclib::DisjointSet::init_by_size(self.boxes.len());
        let mut counter = 0;
        let mut ans = None;
        for &(_dist, i, j) in distances.iter() {
            if ds.union(i, j) {
                counter += 1;
                if counter == self.boxes.len() - 1 {
                    ans = Some(self.boxes[i].0 * self.boxes[j].0);
                    break;
                }
            }
        }

        aoclib::output(ans.unwrap())
    }
}

#[derive(Debug)]
struct Point(usize, usize, usize);

impl Point {
    fn dist_to(&self, other: &Point) -> f64 {
        let x = self.0.abs_diff(other.0) as f64;
        let y = self.1.abs_diff(other.1) as f64;
        let z = self.2.abs_diff(other.2) as f64;
        (x * x + y * y + z * z).sqrt()
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trip = s.split(',');
        Ok(Point(
            trip.next().unwrap().parse().unwrap(),
            trip.next().unwrap().parse().unwrap(),
            trip.next().unwrap().parse().unwrap(),
        ))
    }
}

mod disjoint_set {}
