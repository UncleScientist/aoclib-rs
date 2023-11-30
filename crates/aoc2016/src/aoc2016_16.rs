use aoclib::Runner;

const PUZZLE_INPUT: &str = "11101000110010100";

pub struct Aoc2016_16 {
    scram: Scramble,
}

impl Aoc2016_16 {
    pub fn new() -> Self {
        Self {
            scram: Scramble::new("".to_string()),
        }
    }
}

impl Runner for Aoc2016_16 {
    fn name(&self) -> (usize, usize) {
        (2016, 16)
    }

    fn parse(&mut self) {
        self.scram = Scramble::new(PUZZLE_INPUT.to_string());
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.scram.checksum(272))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.scram.checksum(35_651_584))
    }
}

struct Scramble {
    input: String,
}

impl Scramble {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn checksum(&self, len: usize) -> String {
        let mut result = self.input.clone();

        while result.len() < len {
            let rev = result
                .chars()
                .rev()
                .map(|ch| match ch {
                    '0' => '1',
                    '1' => '0',
                    _ => panic!("invalid input"),
                })
                .collect::<String>();
            result = format!("{result}0{rev}");
        }

        result.truncate(len);

        while result.len() % 2 == 0 {
            let slice = result.chars().collect::<Vec<char>>();
            result = slice
                .chunks(2)
                .map(|pair| if pair[0] == pair[1] { '1' } else { '0' })
                .collect::<String>();
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let scram = Scramble::new("10000".to_string());
        assert_eq!("01100".to_string(), scram.checksum(20));
    }
}
