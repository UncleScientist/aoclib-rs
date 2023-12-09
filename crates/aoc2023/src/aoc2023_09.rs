use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_09 {
    nums: Vec<Vec<i64>>,
}

impl Aoc2023_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_09 {
    fn name(&self) -> (usize, usize) {
        (2023, 9)
    }

    fn parse(&mut self) {
        self.nums = aoclib::numbers("input/2023-09.txt", ' ');
        /*
        for testdata in [
            [0i64, 3, 6, 9, 12, 15],
            [1, 3, 6, 10, 15, 21],
            [10, 13, 16, 21, 30, 45],
        ] {
            println!("{}", solve(&testdata));
        }
        */
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.nums.iter().map(|nums| solve(nums)).sum::<i64>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

fn solve(nums: &[i64]) -> i64 {
    let mut ans = *nums.last().unwrap();
    let mut diffs = nums
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<i64>>();
    ans += diffs.last().unwrap();

    loop {
        diffs = diffs
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<i64>>();
        if diffs[0] == diffs[1] && diffs.iter().all(|num| *num == 0) {
            return ans;
        }
        ans += diffs.last().unwrap();
    }
}
