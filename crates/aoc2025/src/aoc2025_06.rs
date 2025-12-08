use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_06 {
    rows: Vec<Vec<usize>>,
    ops: Vec<Op>,
}

impl Aoc2025_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_06 {
    fn name(&self) -> (usize, usize) {
        (2025, 6)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-06.txt");
        for line in &lines[0..lines.len() - 1] {
            self.rows.push(
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            );
        }
        self.ops = lines[lines.len() - 1]
            .split_whitespace()
            .map(|op| if op == "*" { Op::Multiply } else { Op::Add })
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        assert_eq!(self.rows[0].len(), self.ops.len());
        let mut total = 0;
        for i in 0..self.ops.len() {
            match self.ops[i] {
                Op::Multiply => {
                    let mut ans = 1;
                    for entry in &self.rows {
                        ans *= entry[i];
                    }
                    total += ans;
                }
                Op::Add => {
                    let mut ans = 0;
                    for entry in &self.rows {
                        ans += entry[i];
                    }
                    total += ans;
                }
            }
        }
        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Multiply,
    Add,
}
