use aoclib::read_lines;
use aoclib::Runner;

pub struct Aoc2016_09 {
    input: String,
}

impl Aoc2016_09 {
    pub fn new() -> Self {
        Self {
            input: "".to_string(),
        }
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

trait Explode {
    fn explode(&self) -> usize;
}

impl Explode for String {
    fn explode(&self) -> usize {
        let mut answer = 0;
        let mut loopstr = self.clone();

        while let Some((left, right)) = loopstr.split_once('(') {
            answer += left.len();

            let (code, rest) = right.split_once(')').unwrap();
            let (len, count) = code.split_once('x').unwrap();
            let len: usize = len.parse().unwrap();
            let count: usize = count.parse().unwrap();
            let (compressed, remainder) = rest.split_at(len);
            answer += count * compressed.to_string().explode();

            loopstr = remainder.to_string();
        }

        answer + loopstr.len()
    }
}

impl Runner for Aoc2016_09 {
    fn name(&self) -> (usize, usize) {
        (2016, 9)
    }

    fn parse(&mut self) {
        self.input = read_lines("input/2016-09.txt")[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.input.decompress())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.input.explode())
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

    #[test]
    fn explode_1() {
        assert_eq!(9, "(3x3)XYZ".to_string().explode());
    }

    #[test]
    fn explode_big() {
        assert_eq!(
            "XABCABCABCABCABCABCY".len(),
            "X(8x2)(3x3)ABCY".to_string().explode()
        );
    }

    #[test]
    fn explode_huge() {
        assert_eq!(
            241920,
            "(27x12)(20x12)(13x14)(7x10)(1x12)A".to_string().explode()
        );
    }

    #[test]
    fn explode_mini() {
        assert_eq!(
            445,
            "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"
                .to_string()
                .explode()
        );
    }
}
