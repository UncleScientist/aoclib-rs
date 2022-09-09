use crate::Runner;

const TARGET: u64 = 34_000_000;

pub struct Aoc2015_20 {
    houses: Vec<u64>,
}

impl Aoc2015_20 {
    pub fn new() -> Self {
        Self { houses: Vec::new() }
    }
}

impl Runner for Aoc2015_20 {
    fn name(&self) -> (usize, usize) {
        (2015, 20)
    }

    fn parse(&mut self) {
        let mut limit = 1_000_000;
        self.houses.resize(limit, 0);
        for elf in 1..limit {
            for visited in (elf..limit).step_by(elf) {
                self.houses[visited] += elf as u64 * 10;
                if self.houses[visited] >= TARGET && visited < limit {
                    limit = visited;
                    break;
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        for (idx, house) in self.houses.iter().enumerate() {
            if *house >= TARGET {
                return crate::output(idx);
            }
        }
        crate::output("can't find a good home")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
