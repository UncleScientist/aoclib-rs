use crate::Runner;

pub struct Aoc2016_05 {
    prefix: String,
    digest: Vec<md5::Digest>,
    index: usize,
}

impl Aoc2016_05 {
    pub fn new() -> Self {
        Self {
            prefix: "ojvtpuvg".to_string(),
            digest: Vec::new(),
            index: 0,
        }
    }
}

impl Runner for Aoc2016_05 {
    fn name(&self) -> (usize, usize) {
        (2016, 5)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let mut result = "".to_string();
        while let Some(d) = self.next() {
            result.push(val_to_char((d[2] & 0xf) as u8));
            self.digest.push(d);
            if result.len() == 8 {
                println!("result is 8");
                break;
            }
        }

        crate::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut ary = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
        let mut count = 0;
        for d in self.digest.iter() {
            let idx = (d[2] & 0xf) as usize;
            if idx >= 8 {
                continue;
            }

            let val = ((d[3] & 0xf0) >> 4) as u8;
            if ary[idx] == ' ' {
                ary[idx] = val_to_char(val);
                count += 1;
            }
        }

        while count < 8 {
            let d = self.next().unwrap();
            let idx = (d[2] & 0xf) as usize;
            if idx >= 8 {
                continue;
            }

            let val = ((d[3] & 0xf0) >> 4) as u8;
            if ary[idx] == ' ' {
                ary[idx] = val_to_char(val);
                count += 1;
            }
        }

        let result: String = ary.iter().collect();
        crate::output(result)
    }
}

impl Iterator for Aoc2016_05 {
    type Item = md5::Digest;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            let digest = md5::compute(format!("{}{}", self.prefix, self.index));
            self.index += 1;

            if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
                return Some(digest);
            }
        }
    }
}

fn val_to_char(v: u8) -> char {
    if v < 10 {
        (b'0' + v) as char
    } else {
        (b'a' + v - 10) as char
    }
}
