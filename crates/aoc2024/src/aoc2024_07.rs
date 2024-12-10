use std::str::FromStr;

use aoclib::Runner;

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
        self.equations = lines.iter().map(|line| line.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            self.equations
                .iter()
                .filter_map(|eq| eq.solution())
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
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
    fn solution(&self) -> Option<usize> {
        let mut ops = vec!['+'; self.list.len() - 1];
        loop {
            let total =
                ops.iter()
                    .zip(&self.list[1..])
                    .fold(self.list[0], |val, (op, num)| match op {
                        '+' => val + *num,
                        '*' => val * *num,
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
        assert_eq!(Some(292), eq.solution());
    }
}
