use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_02 {
    nums: Vec<Vec<i64>>,
}

impl Aoc2024_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_02 {
    fn name(&self) -> (usize, usize) {
        (2024, 2)
    }

    fn parse(&mut self) {
        self.nums = aoclib::numbers("input/2024-02.txt", ' ');
        // self.nums = aoclib::numbers("test.txt", ' ');
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.nums.iter().filter(|list| is_safe(list)).count())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut num_safe = 0;
        for report in &self.nums {
            if is_safe(report) {
                num_safe += 1;
            } else {
                for idx in 0..report.len() {
                    let mut r2 = report.clone();
                    r2.remove(idx);
                    if is_safe(&r2) {
                        num_safe += 1;
                        break;
                    }
                }
            }
        }
        aoclib::output(num_safe)
    }
}

fn is_safe(nums: &[i64]) -> bool {
    nums.is_sorted_by(|a, b| a < b && (1..=3).contains(&(b - a)))
        || nums.is_sorted_by(|a, b| b < a && (1..=3).contains(&(a - b)))
}
