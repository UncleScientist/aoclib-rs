use std::str::FromStr;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_02 {
    ranges: Vec<Range>,
}

impl Aoc2025_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_02 {
    fn name(&self) -> (usize, usize) {
        (2025, 2)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-02.txt");
        self.ranges = lines[0]
            .split(',')
            .map(|range| range.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.ranges.iter().map(Range::sum_invalid).sum::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        let solution = self
            .ranges
            .iter()
            .map(Range::sum_multi_invalid)
            .sum::<usize>();
        aoclib::output(solution)
    }
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn sum_invalid(&self) -> usize {
        (self.start..=self.end)
            .filter(|num| {
                let half_digits = num.ilog10().div_ceil(2);
                let mod_val = 10usize.pow(half_digits);
                let lower_half = num % mod_val;
                let upper_half = num / mod_val;
                lower_half == upper_half
            })
            .sum()
    }

    fn sum_multi_invalid(&self) -> usize {
        let mut invalid_sum = 0;

        for num in self.start..=self.end {
            let half_digits = num.ilog10().div_ceil(2);
            for digit_count in 1..=half_digits {
                let mod_val = 10usize.pow(digit_count);
                let last_n_digits = num % mod_val;
                let mut test_num = num / mod_val;
                if last_n_digits == 0 || last_n_digits.ilog(10) + 1 != digit_count {
                    continue;
                }
                let mut found = true;
                while test_num > 0 {
                    found = test_num % mod_val == last_n_digits;
                    if !found {
                        break;
                    }
                    test_num /= mod_val;
                }

                if found {
                    invalid_sum += num;
                    break;
                }
            }
        }

        invalid_sum
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').unwrap();
        Ok(Range {
            start: left.parse().unwrap(),
            end: right.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiple_invalids1() {
        let range = Range {
            start: 824824821,
            end: 824824827,
        };
        assert_eq!(824824824, range.sum_multi_invalid());
    }

    #[test]
    fn test_multiple_invalids2() {
        let range = Range { start: 11, end: 22 };
        assert_eq!(33, range.sum_multi_invalid());
    }

    #[test]
    fn test_multiple_invalids3() {
        let range = Range {
            start: 222220,
            end: 222224,
        };
        assert_eq!(222222, range.sum_multi_invalid());
    }

    #[test]
    fn test_multiple_invalids4() {
        let range = Range {
            start: 565653,
            end: 565659,
        };
        assert_eq!(565656, range.sum_multi_invalid());
    }

    #[test]
    fn test_multiple_invalids5() {
        let range = Range {
            start: 95,
            end: 115,
        };
        assert_eq!(99 + 111, range.sum_multi_invalid());
    }

    #[test]
    fn test_multiple_invalids6() {
        let range = Range {
            start: 998,
            end: 1012,
        };
        assert_eq!(999 + 1010, range.sum_multi_invalid());
    }

    #[test]
    fn test_multiple_invalids7() {
        let range = Range {
            start: 70701,
            end: 70710,
        };
        assert_eq!(0, range.sum_multi_invalid());
    }
}
