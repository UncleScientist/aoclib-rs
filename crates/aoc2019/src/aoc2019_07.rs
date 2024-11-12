use aoclib::{Permutations, Runner};

use crate::intcode::Intcode;

#[derive(Default)]
pub struct Aoc2019_07 {
    computer: Intcode,
}

impl Aoc2019_07 {
    pub fn new() -> Self {
        Self::default()
    }

    fn run_sequence(&mut self, seq: &[i64]) -> i64 {
        let mut data = 0;
        for num in seq {
            self.computer.reset();
            self.computer.push(*num);
            self.computer.push(data);
            data = self.computer.run_until_output().unwrap();
        }

        data
    }
}

impl Runner for Aoc2019_07 {
    fn name(&self) -> (usize, usize) {
        (2019, 7)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2019-07.txt");
        self.computer = Intcode::new(&lines[0]);
    }

    fn part1(&mut self) -> Vec<String> {
        let perms = vec![0i64, 1, 2, 3, 4].permutations();

        aoclib::output(perms.map(|perm| self.run_sequence(&perm)).max().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut aoc = Aoc2019_07::default();
        aoc.computer = Intcode::new("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");

        assert_eq!(43210, aoc.run_sequence(&[4, 3, 2, 1, 0]));
    }

    #[test]
    fn example2() {
        let mut aoc = Aoc2019_07::default();
        aoc.computer = Intcode::new(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );

        assert_eq!(54321, aoc.run_sequence(&[0, 1, 2, 3, 4]));
    }

    #[test]
    fn example3() {
        let mut aoc = Aoc2019_07::default();
        aoc.computer = Intcode::new(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );

        assert_eq!(65210, aoc.run_sequence(&[1, 0, 4, 3, 2]));
    }
}
