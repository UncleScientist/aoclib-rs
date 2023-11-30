use aoclib::Runner;

#[derive(PartialEq)]
enum Part {
    CheckPart1,
    CheckPart2,
}

pub struct Aoc2015_04 {
    lowest: usize,
}

impl Aoc2015_04 {
    pub fn new() -> Self {
        Self { lowest: 0 }
    }

    fn find_prefix(&mut self, check: Part) -> Vec<String> {
        loop {
            self.lowest += 1;
            let digest = md5::compute(format!("{PREFIX}{}", self.lowest));

            // 00_00_0abc...
            if digest[0] != 0 || digest[1] != 0 {
                continue;
            }

            if check == Part::CheckPart1 && (digest[2] & 0xf0) != 0 {
                continue;
            }

            if check == Part::CheckPart2 && digest[2] != 0 {
                continue;
            }

            return aoclib::output(self.lowest);
        }
    }
}

const PREFIX: &str = "ckczppom";

impl Runner for Aoc2015_04 {
    fn parse(&mut self) {}

    fn name(&self) -> (usize, usize) {
        (2015, 4)
    }

    fn part1(&mut self) -> Vec<String> {
        self.find_prefix(Part::CheckPart1)
    }

    fn part2(&mut self) -> Vec<String> {
        self.find_prefix(Part::CheckPart2)
    }
}
