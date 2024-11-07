use aoclib::Runner;

const RANGE: &str = "109165-576723";

#[derive(Default)]
pub struct Aoc2019_04 {
    low: usize,
    high: usize,
}

impl Aoc2019_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_04 {
    fn name(&self) -> (usize, usize) {
        (2019, 4)
    }

    fn parse(&mut self) {
        let (low, high) = RANGE.split_once('-').unwrap();
        self.low = low.parse::<usize>().unwrap();
        self.high = high.parse::<usize>().unwrap();
    }

    fn part1(&mut self) -> Vec<String> {
        let count = (self.low..=self.high).filter_map(validate_num).count();
        aoclib::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

fn validate_num(num: usize) -> Option<()> {
    let digits = format!("{num}").chars().collect::<Vec<_>>();

    let increasing = digits.windows(2).all(|w| w[0] <= w[1]);
    let double = digits.windows(2).any(|w| w[0] == w[1]);

    if increasing && double {
        Some(())
    } else {
        None
    }
}
