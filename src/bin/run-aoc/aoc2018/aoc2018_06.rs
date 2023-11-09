use std::cmp::Ordering;

use aoclib::Point;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_06 {
    points: Vec<Point<i64>>,
    min: Point<i64>,
    max: Point<i64>,
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

        for l in &lines {
            let p = l.parse::<Point<i64>>().unwrap();
            self.min = self.min.min(&p);
            self.max = self.max.max(&p);
            self.points.push(p);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut map = vec![0usize; self.points.len()];
        let mut infinite = vec![false; self.points.len()];

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
                    infinite[closest_point] = true;
                    map[closest_point] = 0;
                } else if !infinite[closest_point] {
                    map[closest_point] += 1;
                }
            }
        }

        crate::output(map.iter().max().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;

        for x in self.min.x..=self.max.x {
            for y in self.min.y..=self.max.y {
                let total_dist = self.points.iter().map(|p| p.dist(x, y)).sum::<i64>();
                total += (total_dist < 10_000) as i32;
            }
        }

        crate::output(total)
    }
}
