use std::{cmp::Ordering, str::FromStr};

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_06 {
    points: Vec<Point>,
    min: Point,
    max: Point,
}

impl Aoc2018_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_06 {
    fn name(&self) -> (usize, usize) {
        (2018, 6)
    }

    fn parse(&mut self) {
        let _lines = ["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
        let lines = aoclib::read_lines("input/2018-06.txt");

        self.min.x = i64::MAX;
        self.min.y = i64::MAX;

        for l in lines {
            let p = Point::from_str(&l).unwrap();
            self.min = self.min.min(&p);
            self.max = self.max.max(&p);
            self.points.push(p);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut map = vec![0usize; self.points.len()];

        for x in self.min.x..=self.max.x {
            for y in self.min.y..=self.max.y {
                let mut closest_point = self.points.len();
                let mut min_dist = self.max.x + self.max.y;
                let mut found = false;

                for i in 0..self.points.len() {
                    let dist = self.points[i].dist(x, y);
                    match dist.cmp(&min_dist) {
                        Ordering::Less => {
                            closest_point = i;
                            min_dist = dist;
                            found = true;
                        }
                        Ordering::Equal => found = false,
                        Ordering::Greater => {}
                    }
                }

                if !found {
                    continue;
                }

                if x == self.min.x || x == self.max.x || y == self.min.y || y == self.max.y {
                    self.points[closest_point].infinite = true;
                    map[closest_point] = 0;
                } else if !self.points[closest_point].infinite {
                    map[closest_point] += 1;
                }
            }
        }

        crate::output(map.iter().max().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    infinite: bool,
}

impl Point {
    fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            infinite: false,
        }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            infinite: false,
        }
    }

    fn dist(&self, x: i64, y: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(p: &str) -> Result<Self, Self::Err> {
        let (x, y) = p.split_once(", ").ok_or("input file corrupt")?;
        Ok(Self {
            x: x.parse().map_err(|_| "x value corrupt")?,
            y: y.parse().map_err(|_| "y value corrupt")?,
            infinite: false,
        })
    }
}
