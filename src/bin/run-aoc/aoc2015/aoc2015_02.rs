use crate::Runner;
use aoclib_rs::numbers;

pub struct Present([u32; 3]);

impl Present {
    fn new(v: &[u32]) -> Self {
        Self([v[0], v[1], v[2]])
    }

    fn surface_area(&self) -> u32 {
        2 * self.0[0] * self.0[1] + 2 * self.0[0] * self.0[2] + 2 * self.0[1] * self.0[2]
    }

    fn slack(&self) -> u32 {
        self.0[0] * self.0[1]
    }

    fn ribbon(&self) -> u32 {
        2 * self.0[0] + 2 * self.0[1] + self.0[0] * self.0[1] * self.0[2]
    }
}

pub struct Aoc2015_02 {
    prez: Vec<Present>,
}

impl Aoc2015_02 {
    pub fn new() -> Self {
        let mut data = numbers("input/2015-02.txt", 'x');
        let mut prez = Vec::new();
        for d in &mut data {
            d.sort();
            prez.push(Present::new(&d));
        }

        Self { prez }
    }
}

impl Runner for Aoc2015_02 {
    fn name(&self) -> String {
        "2015 Day 2".to_string()
    }

    fn part1(&mut self) -> Vec<String> {
        vec![format!(
            "{}",
            self.prez
                .iter()
                .map(|p| p.surface_area() + p.slack())
                .sum::<u32>()
        )]
    }

    fn part2(&mut self) -> Vec<String> {
        vec![format!(
            "{}",
            self.prez.iter().map(|p| p.ribbon()).sum::<u32>()
        )]
    }
}
