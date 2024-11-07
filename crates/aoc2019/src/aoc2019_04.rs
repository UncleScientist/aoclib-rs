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
        let count = (self.low..=self.high)
            .filter_map(validate_single_double)
            .count();
        aoclib::output(count)
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

fn validate_single_double(num: usize) -> Option<()> {
    let digits = format!("{num}").chars().collect::<Vec<_>>();

    if !digits.windows(2).all(|w| w[0] <= w[1]) {
        return None;
    }

    let mut iter = digits.iter();
    let mut prev = iter.next().unwrap();
    let mut count = 1;
    while let Some(digit) = iter.next() {
        if prev == digit {
            count += 1;
        } else {
            if count == 2 {
                return Some(());
            }
            count = 1;
        }
        prev = digit;
    }

    if count == 2 {
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(validate_single_double(112233).is_some());
    }

    #[test]
    fn test2() {
        assert!(validate_single_double(123444).is_none());
    }

    #[test]
    fn test3() {
        assert!(validate_single_double(111122).is_some());
    }
}
