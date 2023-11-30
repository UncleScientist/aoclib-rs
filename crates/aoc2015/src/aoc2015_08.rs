use aoclib::read_lines;

use aoclib::Runner;

pub struct Aoc2015_08 {
    lines: Vec<String>,
}

impl Aoc2015_08 {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn encode_string(s: &String) -> (usize, usize) {
        (
            2 + s
                .chars()
                .map(|c| if matches!(c, '"' | '\\') { 2 } else { 1 })
                .sum::<usize>(),
            s.len(),
        )
    }

    fn calc_string(s: &String) -> (usize, usize) {
        let mut escaped = 0;
        let v = s.chars().collect::<Vec<char>>();
        let mut index = 1;
        while index < v.len() - 1 {
            escaped += 1;
            if v[index] == '\\' {
                if v[index + 1] == 'x' {
                    index += 4;
                } else {
                    index += 2;
                }
            } else {
                index += 1;
            }
        }

        (s.len(), escaped)
    }
}

impl Runner for Aoc2015_08 {
    fn parse(&mut self) {
        self.lines = read_lines("input/2015-08.txt");
    }

    fn name(&self) -> (usize, usize) {
        (2015, 8)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        let mut escaped = 0;
        for l in &self.lines {
            let (t, e) = Aoc2015_08::calc_string(l);
            total += t;
            escaped += e;
        }
        aoclib::output(total - escaped)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        let mut encoded = 0;
        for l in &self.lines {
            let (e, t) = Aoc2015_08::encode_string(l);
            total += t;
            encoded += e;
        }
        aoclib::output(encoded - total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_without_escapes() {
        assert_eq!((7, 5), Aoc2015_08::calc_string(&"\"hello\"".to_string()));
    }

    #[test]
    fn calc_encode_blank_string() {
        assert_eq!((6, 2), Aoc2015_08::encode_string(&"\"\"".to_string()));
    }

    #[test]
    fn encode_basic_string() {
        assert_eq!((9, 5), Aoc2015_08::encode_string(&"\"abc\"".to_string()));
    }

    #[test]
    fn encode_hex_value() {
        assert_eq!((11, 6), Aoc2015_08::encode_string(&"\"\\x27\"".to_string()));
    }

    #[test]
    fn encode_embedded_quote() {
        assert_eq!(
            (16, 10),
            Aoc2015_08::encode_string(&"\"aaa\\\"aaa\"".to_string())
        );
    }
}
