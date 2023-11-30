use aoclib::Runner;

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

        aoclib::output(((elf_count - prev) * 2) % elf_count + 1)
    }

    fn part2(&mut self) -> Vec<String> {
        let elf_count = 3018458u32;
        let mut survivor = 1;
        while survivor * 3 < elf_count {
            survivor *= 3;
        }
        if elf_count < 2 * survivor {
            survivor = elf_count - survivor;
        } else {
            survivor = 2 * elf_count - 3 * survivor;
        }
        aoclib::output(survivor)
    }
}
