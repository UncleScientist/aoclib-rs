use aoclib::Runner;

#[derive(Debug, Clone)]
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
        aoclib::output(solution_for_discs(&self.discs))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut additional = self.discs.clone();
        additional.push(Disc::new(self.discs.len() + 1, 0, 11));
        aoclib::output(solution_for_discs(&additional))
    }
}

fn solution_for_discs(discs: &[Disc]) -> usize {
    let mut time = 0;

    'again: loop {
        for d in discs {
            if !d.zero_at_time(time) {
                time += 1;
                continue 'again;
            }
        }
        break;
    }

    time
}
