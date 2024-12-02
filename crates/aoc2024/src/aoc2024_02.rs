use std::cmp::Ordering;

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
    if match nums[0].cmp(&nums[1]) {
        Ordering::Less => nums.windows(2).any(|pair| pair[0] >= pair[1]),
        Ordering::Equal => false,
        Ordering::Greater => nums.windows(2).any(|pair| pair[0] <= pair[1]),
    } {
        return false;
    }

    !nums
        .windows(2)
        .any(|pair| !(1..=3).contains(&pair[0].abs_diff(pair[1])))
}
