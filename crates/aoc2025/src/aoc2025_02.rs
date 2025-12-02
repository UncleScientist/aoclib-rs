use std::str::FromStr;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_02 {
    ranges: Vec<Range>,
}

impl Aoc2025_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_02 {
    fn name(&self) -> (usize, usize) {
        (2025, 2)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-02.txt");
        self.ranges = lines[0]
            .split(',')
            .map(|range| range.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.ranges.iter().map(Range::sum_invalid).sum::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn sum_invalid(&self) -> usize {
        (self.start..=self.end)
            .filter(|num| {
                let half_digits = num.ilog10().div_ceil(2);
                let mod_val = 10usize.pow(half_digits);
                let lower_half = num % mod_val;
                let upper_half = num / mod_val;
                lower_half == upper_half
            })
            .sum()
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').unwrap();
        Ok(Range {
            start: left.parse().unwrap(),
            end: right.parse().unwrap(),
        })
    }
}
