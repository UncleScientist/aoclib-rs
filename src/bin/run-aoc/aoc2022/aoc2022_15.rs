use std::collections::HashSet;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_15 {
    sensors: Vec<Sensor>,
    part1row: i64,
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
        crate::output("unsolved")
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
