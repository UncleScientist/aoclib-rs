use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_11 {
    steps: Vec<Dir>,
    max: i32,
}

impl Aoc2017_11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_11 {
    fn name(&self) -> (usize, usize) {
        (2017, 11)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-11.txt");

        self.steps = lines[0].split(',').map(Dir::parse).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut x = 0i32;
        let mut y = 0i32;
        for step in &self.steps {
            match step {
                Dir::North => y -= 1,
                Dir::Northeast => {
                    x += 1;
                    y -= 1;
                }
                Dir::Southeast => x += 1,
                Dir::South => y += 1,
                Dir::Southwest => {
                    x -= 1;
                    y += 1;
                }
                Dir::Northwest => x -= 1,
            }
            self.max = self.max.max(y.abs().max(x.abs()));
        }
        aoclib::output(y.abs().max(x.abs()))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.max)
    }
}

#[derive(Debug)]
enum Dir {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

impl Dir {
    fn parse(s: &str) -> Self {
        match s {
            "n" => Self::North,
            "ne" => Self::Northeast,
            "se" => Self::Southeast,
            "s" => Self::South,
            "sw" => Self::Southwest,
            "nw" => Self::Northwest,
            _ => panic!("corrupted input file"),
        }
    }
}
