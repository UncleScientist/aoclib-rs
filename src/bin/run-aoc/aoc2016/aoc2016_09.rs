use crate::Runner;
use aoclib::read_lines;

pub struct Aoc2016_09;

impl Aoc2016_09 {
    pub fn new() -> Self {
        Self {}
    }
}

trait Decompress {
    fn decompress(&self) -> usize;
}

impl Decompress for String {
    fn decompress(&self) -> usize {
        if !self.contains('(') {
            return self.len();
        }

        let (left, mut right) = self.split_once('(').unwrap();
        let mut answer = left.len();
        while right.contains(')') {
            let (code, rest) = right.split_once(')').unwrap();
            let (len, count) = code.split_once('x').unwrap();
            let len: usize = len.parse().unwrap();
            let count: usize = count.parse().unwrap();
            answer += len * count;
            let (_, remainder) = rest.split_at(len);
            if !remainder.contains('(') {
                return answer + remainder.len();
            }
            let (left, remainder) = remainder.split_once('(').unwrap();
            answer += left.len();
            right = remainder;
        }

        answer
    }
}

impl Runner for Aoc2016_09 {
    fn name(&self) -> (usize, usize) {
        (2016, 9)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let lines = read_lines("input/2016-09.txt");
        crate::output(lines[0].decompress())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_markers() {
        assert_eq!(6, "ADVENT".to_string().decompress());
    }

    #[test]
    fn repeat_b() {
        assert_eq!(7, "A(1x5)BC".to_string().decompress());
    }

    #[test]
    fn xyz_9() {
        assert_eq!(9, "(3x3)XYZ".to_string().decompress());
    }

    #[test]
    fn double_decompress() {
        assert_eq!(11, "A(2x2)BCD(2x2)EFG".to_string().decompress());
    }

    #[test]
    fn juxtaposed() {
        assert_eq!(6, "(6x1)(1x3)A".to_string().decompress());
    }

    #[test]
    fn prefix_juxtaposed() {
        assert_eq!(18, "X(8x2)(3x3)ABCY".to_string().decompress());
    }
}
