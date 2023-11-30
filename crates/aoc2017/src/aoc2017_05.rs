use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_05 {
    jumps: Vec<i32>,
}

impl Aoc2017_05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_05 {
    fn name(&self) -> (usize, usize) {
        (2017, 5)
    }

    fn parse(&mut self) {
        self.jumps = aoclib::read_numbers("input/2017-05.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut jumps = self.jumps.clone();

        let mut ip = 0i32;
        let mut count = 0;
        while ip < jumps.len() as i32 {
            let offset = jumps[ip as usize];
            jumps[ip as usize] += 1;
            ip += offset;
            count += 1;
        }
        aoclib::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut jumps = self.jumps.clone();

        let mut ip = 0i32;
        let mut count = 0;
        while ip < jumps.len() as i32 {
            let offset = jumps[ip as usize];
            if offset >= 3 {
                jumps[ip as usize] -= 1;
            } else {
                jumps[ip as usize] += 1;
            }
            ip += offset;
            count += 1;
        }
        aoclib::output(count)
    }
}
