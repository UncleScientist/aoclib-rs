use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_01 {
    nums: Vec<[i64; 2]>,
}

impl Aoc2024_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_01 {
    fn name(&self) -> (usize, usize) {
        (2024, 1)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-01.txt");
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();
        for line in lines {
            let (left, right) = line.split_once("   ").unwrap();
            let left = left.parse::<i64>().unwrap();
            let right = right.parse::<i64>().unwrap();
            left_list.push(left);
            right_list.push(right);
        }
        left_list.sort();
        right_list.sort();
        self.nums.extend(
            left_list
                .iter()
                .zip(right_list.iter())
                .map(|(a, b)| [*a, *b]),
        );
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            self.nums
                .iter()
                .map(|pair| pair[0].abs_diff(pair[1]))
                .sum::<u64>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut score = 0;
        for pair in &self.nums {
            let left = pair[0];
            let count = self.nums.iter().filter(|pair| pair[1] == left).count();
            score += left * count as i64;
        }
        aoclib::output(score)
    }
}
