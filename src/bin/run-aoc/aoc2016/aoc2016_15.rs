use crate::Runner;

#[derive(Debug)]
struct Disc {
    num: usize,
    phase: usize,
    positions: usize,
}

impl Disc {
    fn new(num: usize, phase: usize, positions: usize) -> Self {
        Self {
            num,
            phase,
            positions,
        }
    }

    fn zero_at_time(&self, time: usize) -> bool {
        (time + self.num + self.phase) % self.positions == 0
    }
}

pub struct Aoc2016_15 {
    discs: Vec<Disc>,
}

impl Aoc2016_15 {
    pub fn new() -> Self {
        Self { discs: Vec::new() }
    }
}

impl Runner for Aoc2016_15 {
    fn name(&self) -> (usize, usize) {
        (2016, 15)
    }

    fn parse(&mut self) {
        /*
         * test input
         * self.discs.push(Disc::new(1, 4, 5));
         * self.discs.push(Disc::new(2, 1, 2));
         */
        for (idx, l) in aoclib::read_lines("input/2016-15.txt").iter().enumerate() {
            let words = l.split(' ').collect::<Vec<&str>>();
            self.discs.push(Disc::new(
                idx + 1,
                words[11]
                    .split_once('.')
                    .unwrap()
                    .0
                    .parse::<usize>()
                    .unwrap(),
                words[3].parse::<usize>().unwrap(),
            ));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut time = 0;

        'again: loop {
            for d in &self.discs {
                if !d.zero_at_time(time) {
                    time += 1;
                    continue 'again;
                }
            }
            break;
        }

        crate::output(time)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
