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
        let nums = [
            "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven",
            "7", "eight", "8", "nine", "9",
        ];
        let mut total = 0;

        for line in &self.lines {
            let mut first = None;
            'out: for i in 0..line.len() {
                for (index, num) in nums.iter().enumerate() {
                    if i + num.len() > line.len() {
                        continue;
                    }
                    if line[i..i + num.len()] == **num {
                        first = Some(1 + index / 2);
                        break 'out;
                    }
                }
            }
            let Some(first) = first else {
                panic!("invalid input");
            };

            let mut last = None;
            'out: for i in (0..line.len()).rev() {
                for (index, num) in nums.iter().enumerate() {
                    if i + num.len() > line.len() {
                        continue;
                    }
                    if line[i..i + num.len()] == **num {
                        last = Some(1 + index / 2);
                        break 'out;
                    }
                }
            }
            let Some(last) = last else {
                panic!("invalid input");
            };

            total += 10 * first as i32 + last as i32;
        }

        aoclib::output(total)
    }
}
