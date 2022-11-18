use crate::Runner;

pub struct Aoc2016_19;

impl Aoc2016_19 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2016_19 {
    fn name(&self) -> (usize, usize) {
        (2016, 19)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let elf_count = 3018458u32;
        let prev = elf_count.next_power_of_two() >> 1;

        crate::output(((elf_count - prev) * 2) % elf_count + 1)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
