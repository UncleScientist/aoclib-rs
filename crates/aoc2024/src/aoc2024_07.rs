use std::str::FromStr;

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Aoc2024_07 {
    equations: Vec<Equation>,
}

impl Aoc2024_07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_07 {
    fn name(&self) -> (usize, usize) {
        (2024, 7)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-07.txt");
        self.equations = lines.parse_lines();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            self.equations
                .iter()
                .filter_map(|eq| eq.solution(false))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(
            self.equations
                .iter()
                .filter_map(|eq| eq.solution(true))
                .sum::<usize>(),
        )
    }
}

#[derive(Debug)]
struct Equation {
    value: usize,
    list: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let value = left.parse().unwrap();
        let list = right.split(' ').map(|num| num.parse().unwrap()).collect();

        Ok(Self { value, list })
    }
}

impl Equation {
    fn solution(&self, part2: bool) -> Option<usize> {
        let mut ops = vec!['+'; self.list.len() - 1];
        loop {
            let total =
                ops.iter()
                    .zip(&self.list[1..])
                    .fold(self.list[0], |val, (op, num)| match op {
                        '+' => val + *num,
                        '*' => val * *num,
                        '|' => {
                            if *num < 10 {
                                (val * 10) + *num
                            } else if *num < 100 {
                                (val * 100) + *num
                            } else if *num < 1000 {
                                (val * 1000) + *num
                            } else {
                                panic!("number too large");
                            }
                        }
                        _ => panic!("invalid op {op}"),
                    });
            if total == self.value {
                return Some(total);
            }
            let mut pos = ops.len() - 1;
            loop {
                if ops[pos] == '+' {
                    ops[pos] = '*';
                    break;
                } else if part2 && ops[pos] == '*' {
                    ops[pos] = '|';
                    break;
                } else if pos == 0 {
                    return None;
                }
                ops[pos] = '+';
                pos -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let eq: Equation = "292: 11 6 16 20".parse().unwrap();
        assert_eq!(Some(292), eq.solution(false));
    }
}
