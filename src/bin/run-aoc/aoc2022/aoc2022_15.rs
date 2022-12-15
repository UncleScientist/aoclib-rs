use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_15 {
    sensors: Vec<Sensor>,
    part1row: i64,
    part2max: i64,
}

impl Aoc2022_15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_15 {
    fn name(&self) -> (usize, usize) {
        (2022, 15)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-15.txt");
        self.part1row = 2000000;
        self.part2max = 4000000;

        for line in lines {
            self.sensors.push(Sensor::parse(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut coverage = HashSet::new();

        for s in &self.sensors {
            let radius = s.radius();
            let dist = (s.loc.1 - self.part1row).abs();
            if dist > radius {
                continue;
            }
            let remainder = radius - dist;
            let left_x = s.loc.0 - remainder;
            let right_x = s.loc.0 + remainder;

            for x in left_x..=right_x {
                coverage.insert(x);
            }
        }

        let beacons: HashSet<i64> = HashSet::from_iter(
            self.sensors
                .iter()
                .filter(|s| s.beacon.1 == self.part1row)
                .map(|s| s.beacon.0),
        );

        crate::output(coverage.len() - beacons.len())
    }

    fn part2(&mut self) -> Vec<String> {
        // row[0]    vec![Range { low..high }, Range {low..high}....]
        // row[1]    vec![Range { 0..part2max} }
        // row[2]

        let mut rowdata = vec![vec![0..=self.part2max]; self.part2max as usize + 1];
        for s in &self.sensors {
            let radius = s.radius();
            let top = 0.max(s.loc.1 - radius);
            let bottom = self.part2max.min(s.loc.1 + radius);

            for row in top..=bottom {
                let dist = (s.loc.1 - row).abs();
                let min_x = 0.max(s.loc.0 - (radius - dist));
                let max_x = self.part2max.min(s.loc.0 + (radius - dist));
                // start............end
                //      min...max
                //              min.......max
                // .......max
                let mut new_range = Vec::new();
                for r in &rowdata[row as usize] {
                    let start = *r.start();
                    if start > max_x {
                        new_range.push(r.clone());
                        continue;
                    }

                    let end = *r.end();
                    if end < min_x {
                        new_range.push(r.clone());
                        continue;
                    }

                    if start < min_x {
                        new_range.push(start..=min_x - 1);
                    }
                    if end > max_x {
                        new_range.push(max_x + 1..=end);
                    }
                }

                rowdata[row as usize] = new_range;
            }
        }

        for (y, r) in rowdata.iter().enumerate() {
            if !r.is_empty() {
                let x = *r[0].start();
                return crate::output(x * 4000000 + y as i64);
            }
        }

        crate::output("utter failure")
    }
}

#[derive(Default, Debug)]
struct Sensor {
    loc: (i64, i64),
    beacon: (i64, i64),
}

impl Sensor {
    fn parse(s: &str) -> Self {
        let (left, beacon) = s.split_once(": closest beacon is at x=").unwrap();
        let (_, sensor) = left.split_once("Sensor at x=").unwrap();

        Self {
            loc: Self::coord(sensor),
            beacon: Self::coord(beacon),
        }
    }

    fn coord(s: &str) -> (i64, i64) {
        let (x, y) = s.split_once(", y=").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }

    fn radius(&self) -> i64 {
        (self.beacon.0 - self.loc.0).abs() + (self.beacon.1 - self.loc.1).abs()
    }
}
