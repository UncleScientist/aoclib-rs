use aoclib::numbers;
use aoclib::Runner;

pub struct Present([u32; 3]);

impl Present {
    fn new(v: &[u32]) -> Self {
        Self([v[0], v[1], v[2]])
    }

    fn surface_area(&self) -> u32 {
        2 * self.0[0] * self.0[1] + 2 * self.0[0] * self.0[2] + 2 * self.0[1] * self.0[2]
    }

    fn slack(&self) -> u32 {
        self.0[0] * self.0[1]
    }

    fn ribbon(&self) -> u32 {
        2 * self.0[0] + 2 * self.0[1] + self.0[0] * self.0[1] * self.0[2]
    }
}

pub struct Aoc2015_02 {
    prez: Vec<Present>,
}

impl Aoc2015_02 {
    pub fn new() -> Self {
        Self { prez: Vec::new() }
    }
}

impl Runner for Aoc2015_02 {
    fn parse(&mut self) {
        let mut data = numbers("input/2015-02.txt", 'x');
        for d in &mut data {
            d.sort();
            self.prez.push(Present::new(d));
        }
    }

    fn name(&self) -> (usize, usize) {
        (2015, 2)
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            self.prez
                .iter()
                .map(|p| p.surface_area() + p.slack())
                .sum::<u32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.prez.iter().map(|p| p.ribbon()).sum::<u32>())
    }
}
