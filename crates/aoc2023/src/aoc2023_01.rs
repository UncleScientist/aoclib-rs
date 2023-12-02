use aho_corasick::AhoCorasick;
use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_01 {
    lines: Vec<String>,
}

impl Aoc2023_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_01 {
    fn name(&self) -> (usize, usize) {
        (2023, 1)
    }

    fn parse(&mut self) {
        self.lines = aoclib::read_lines("input/2023-01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;

        for line in &self.lines {
            let nums = line
                .chars()
                .filter(|ch| ch.is_ascii_digit())
                .map(|ch| ch as u8 - b'0')
                .collect::<Vec<_>>();
            let first = *nums.first().unwrap();
            let last = *nums.last().unwrap();

            total += (first as i32) * 10 + last as i32;
        }

        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let nums = [
            "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ];
        let mut total = 0;

        let ac = AhoCorasick::new(nums).unwrap();

        for line in &self.lines {
            let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
            let first = matches.first().unwrap().pattern().as_i32() / 2 + 1;
            let last = matches.last().unwrap().pattern().as_i32() / 2 + 1;

            total += 10 * first + last;
        }

        aoclib::output(total)
    }
}
