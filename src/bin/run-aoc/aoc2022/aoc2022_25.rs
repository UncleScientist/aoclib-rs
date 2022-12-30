use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_25 {
    nums: Vec<String>,
}

impl Aoc2022_25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_25 {
    fn name(&self) -> (usize, usize) {
        (2022, 25)
    }

    fn parse(&mut self) {
        let _lines = aoclib::read_lines("test-input.txt");
        self.nums = aoclib::read_lines("input/2022-25.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(to_snafu(self.nums.iter().map(|s| to_i64(s)).sum()))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("Merry Christmas!")
    }
}

fn to_i64(s: &str) -> i64 {
    let mut result = 0;
    for c in s.chars() {
        match c {
            '0'..='2' => result = (result * 5) + (c as u8 - b'0') as i64,
            '-' => result = result * 5 - 1,
            '=' => result = result * 5 - 2,
            _ => panic!("invalid char '{c}'"),
        }
    }

    result
}

fn to_snafu(mut num: i64) -> String {
    let mut s = String::new();
    while num > 0 {
        s.push("012=-".chars().nth((num % 5) as usize).unwrap());
        num = (num + 2) / 5;
    }
    if s.is_empty() {
        "0".to_string()
    } else {
        s.chars().rev().collect()
    }
}

fn _to_snafu(mut num: i64) -> String {
    let mut result = String::new();

    loop {
        let n = num % 5;
        match n {
            0..=2 => {
                result.push((n as u8 + b'0') as char);
                num /= 5;
            }
            3 => {
                // double minus
                result.push('=');
                num = (num + 2) / 5;
            }
            4 => {
                result.push('-');
                num = (num + 1) / 5;
            }
            _ => panic!("math is broken"),
        }

        if num == 0 {
            break;
        }
    }

    result.chars().rev().collect()
}
