use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_01 {
    module_mass: Vec<usize>,
}

impl Aoc2019_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_01 {
    fn name(&self) -> (usize, usize) {
        (2019, 1)
    }

    fn parse(&mut self) {
        self.module_mass = aoclib::read_numbers("input/day01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            self.module_mass
                .iter()
                .map(|mass| calc_fuel(*mass))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

fn calc_fuel(mass: usize) -> usize {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation() {
        assert_eq!(2, calc_fuel(12));
        assert_eq!(2, calc_fuel(14));
        assert_eq!(654, calc_fuel(1969));
        assert_eq!(33583, calc_fuel(100756));
    }
}
