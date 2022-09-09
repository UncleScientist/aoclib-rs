use crate::Runner;

const TARGET: u64 = 34_000_000;

pub struct Aoc2015_20;

impl Aoc2015_20 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2015_20 {
    fn name(&self) -> (usize, usize) {
        (2015, 20)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let mut limit = 1_000_000;
        let mut houses = vec![0u64; limit];

        for elf in 1..limit {
            for visited in (elf..limit).step_by(elf) {
                houses[visited] += elf as u64 * 10;
                if houses[visited] >= TARGET && visited < limit {
                    limit = visited;
                    break;
                }
            }
        }
        for (idx, house) in houses.iter().enumerate() {
            if *house >= TARGET {
                return crate::output(idx);
            }
        }
        crate::output("can't find a good home")
    }

    fn part2(&mut self) -> Vec<String> {
        let mut limit = 1_000_000;
        let mut houses = vec![0u64; limit];

        for elf in 1..limit {
            let fifty = limit.min(elf * 50);
            for visited in (elf..fifty).step_by(elf) {
                houses[visited] += elf as u64 * 11;
                if houses[visited] >= TARGET && visited < limit {
                    limit = visited;
                    break;
                }
            }
        }
        for (idx, house) in houses.iter().enumerate() {
            if *house >= TARGET {
                return crate::output(idx);
            }
        }
        crate::output("can't find a good home")
    }
}
