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
                .map(|mass| calc_fuel(*mass).unwrap_or(0))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(
            self.module_mass
                .iter()
                .map(|mass| calc_fuel_for_fuel(*mass))
                .sum::<usize>(),
        )
    }
}

fn calc_fuel(mass: usize) -> Option<usize> {
    if mass / 3 < 2 {
        None
    } else {
        Some(mass / 3 - 2)
    }
}

fn calc_fuel_for_fuel(mut mass: usize) -> usize {
    let mut total = 0;

    while let Some(fuel) = calc_fuel(mass) {
        total += fuel;
        mass = fuel;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(2), calc_fuel(12));
        assert_eq!(Some(2), calc_fuel(14));
        assert_eq!(Some(654), calc_fuel(1969));
        assert_eq!(Some(33583), calc_fuel(100756));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(2, calc_fuel_for_fuel(14));
        assert_eq!(966, calc_fuel_for_fuel(1969));
        assert_eq!(50346, calc_fuel_for_fuel(100756));
    }
}
