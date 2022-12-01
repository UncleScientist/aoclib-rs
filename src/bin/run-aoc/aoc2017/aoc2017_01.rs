use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_01 {
    nums: Vec<u64>,
}

impl Aoc2017_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_01 {
    fn name(&self) -> (usize, usize) {
        (2017, 1)
    }

    fn parse(&mut self) {
        self.nums = aoclib::read_single_line("input/2017-01.txt")
            .iter()
            .map(|&ch| ((ch as u8) - b'0') as u64)
            .collect();
        self.nums.push(self.nums[0]);
    }

    fn part1(&mut self) -> Vec<String> {
        let sum = self
            .nums
            .windows(2)
            .filter(|p| p[0] == p[1])
            .map(|p| p[0])
            .sum::<u64>();
        crate::output(sum)
    }

    fn part2(&mut self) -> Vec<String> {
        let len = self.nums.len();
        let sum = 2
            * (0..len / 2)
                .filter(|&p| self.nums[p] == self.nums[p + (len / 2)])
                .map(|p| self.nums[p])
                .sum::<u64>();
        crate::output(sum)
    }
}
