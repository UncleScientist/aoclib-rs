use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_02 {
    nums: Vec<Vec<u64>>,
}

impl Aoc2017_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_02 {
    fn name(&self) -> (usize, usize) {
        (2017, 2)
    }

    fn parse(&mut self) {
        self.nums = aoclib::numbers("input/2017-02.txt", '\t');
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.nums.iter().map(|v| minmax_diff(v)).sum::<u64>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.nums.iter().map(|v| divisible(v)).sum::<u64>())
    }
}

fn minmax_diff(nums: &[u64]) -> u64 {
    let mut min = nums[0];
    let mut max = nums[0];
    for item in nums.iter().skip(1) {
        min = min.min(*item);
        max = max.max(*item);
    }
    max - min
}

fn divisible(nums: &[u64]) -> u64 {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            } else if nums[i] % nums[j] == 0 {
                return nums[i] / nums[j];
            }
        }
    }
    panic!("bad input for finding a solution");
}
