use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_01 {
    records: Vec<Vec<i64>>,
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
        self.records = aoclib::read_num_records("input/2022-01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(
            self.records
                .iter()
                .map(|r| r.iter().sum::<i64>())
                .max()
                .unwrap(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
