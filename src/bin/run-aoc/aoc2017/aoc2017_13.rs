use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_13 {
    scanners: Vec<(usize, usize)>,
}

impl Aoc2017_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_13 {
    fn name(&self) -> (usize, usize) {
        (2017, 13)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-13.txt");

        for line in lines {
            let (layer, depth) = line.split_once(": ").unwrap();
            self.scanners
                .push((layer.parse().unwrap(), depth.parse().unwrap()));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut severity = 0;
        let mut picoseconds = 0;

        for &(layer, depth) in &self.scanners {
            let dist = layer - picoseconds;

            picoseconds += dist;

            if picoseconds % ((depth - 1) * 2) == 0 {
                severity += layer * depth;
            }
        }

        crate::output(severity)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
