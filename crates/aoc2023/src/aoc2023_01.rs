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
            let first = *nums.iter().nth(0).unwrap();
            let last = *nums.iter().last().unwrap();

            total += (first as i32) * 10 + last as i32;
        }

        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
