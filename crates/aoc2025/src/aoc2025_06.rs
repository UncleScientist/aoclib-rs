use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2025_06 {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
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
        self.rows = lines.len();
        self.cols = lines[0].len();
        for line in lines.iter() {
            self.grid.push(line.chars().collect());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut rows: Vec<Vec<usize>> = Vec::new();
        let mut lines = Vec::new();

        for i in 0..self.rows {
            lines.push(self.grid[i].iter().collect::<String>());
        }

        for line in &lines[0..lines.len() - 1] {
            rows.push(
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            );
        }
        let ops = lines[lines.len() - 1]
            .split_whitespace()
            .map(|op| if op == "*" { Op::Multiply } else { Op::Add })
            .collect::<Vec<Op>>();
        assert_eq!(rows[0].len(), ops.len());
        let mut total = 0;
        for i in 0..ops.len() {
            match ops[i] {
                Op::Multiply => {
                    let mut ans = 1;
                    for entry in &rows {
                        ans *= entry[i];
                    }
                    total += ans;
                }
                Op::Add => {
                    let mut ans = 0;
                    for entry in &rows {
                        ans += entry[i];
                    }
                    total += ans;
                }
            }
        }
        assert_eq!(total, 6343365546996);
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
