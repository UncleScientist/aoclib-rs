use crate::Runner;

pub struct Aoc2016_05;

impl Aoc2016_05 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2016_05 {
    fn name(&self) -> (usize, usize) {
        (2016, 5)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let mut result = "".to_string();
        let prefix = "ojvtpuvg";
        let mut count = 0;

        for i in 0.. {
            let digest = md5::compute(format!("{prefix}{i}"));

            if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
                let idx = (digest[2] & 0xf) as u8;
                if idx < 10 {
                    result.push((b'0' + idx) as char);
                } else {
                    result.push((b'a' + idx - 10) as char);
                }
                count += 1;
                if count >= 8 {
                    break;
                }
            }
        }
        crate::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
