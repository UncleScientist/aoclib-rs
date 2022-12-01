use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_01 {
    records: Vec<i64>,
}

impl Aoc2022_01 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_01 {
    fn name(&self) -> (usize, usize) {
        (2022, 1)
    }

    fn parse(&mut self) {
        let records = aoclib::read_num_records("input/2022-01.txt");
        self.records = records
            .iter()
            .map(|r| r.iter().sum::<i64>())
            .collect::<Vec<i64>>();
        self.records.sort_by(|a, b| b.cmp(a));
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.records.first().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.records.iter().take(3).sum::<i64>())
    }
}
