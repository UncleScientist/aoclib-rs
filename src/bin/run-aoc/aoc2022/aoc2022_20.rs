use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_20 {
    data: Vec<(i64, i64)>,
}

impl Aoc2022_20 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_20 {
    fn name(&self) -> (usize, usize) {
        (2022, 20)
    }

    fn parse(&mut self) {
        let _lines = aoclib::read_lines("test-input.txt");
        let lines = aoclib::read_lines("input/2022-20.txt");
        self.data = lines
            .iter()
            .enumerate()
            .map(|n| (n.0 as i64, n.1.parse().unwrap()))
            .collect::<Vec<_>>();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut result = self.data.clone();

        let len = self.data.len() as i64 - 1;

        for d in &self.data {
            let pos = result.iter().position(|n| n == d).unwrap() as i64;
            let mut new_pos = (pos + d.1) % len;

            if new_pos < 0 {
                new_pos += len;
            }

            if new_pos >= len {
                new_pos -= len;
            }

            let val = result.remove(pos as usize);
            result.insert(new_pos as usize, val);
        }

        let zero = result.iter().position(|p| p.1 == 0).unwrap();
        crate::output(
            result[(zero + 1000) % result.len()].1
                + result[(zero + 2000) % result.len()].1
                + result[(zero + 3000) % result.len()].1,
        )
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
