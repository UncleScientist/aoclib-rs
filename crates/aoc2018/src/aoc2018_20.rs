use std::str::Chars;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2018_20 {
    regex: String,
}

impl Aoc2018_20 {
    pub fn new() -> Self {
        Self::default()
    }

    fn longest(&self, iter: &mut Chars) -> usize {
        let mut length = 0;
        let mut max_so_far = 0;

        while let Some(ch) = iter.next() {
            match ch {
                'N' | 'S' | 'E' | 'W' => length += 1,
                '(' => length += self.longest(iter),
                '$' | ')' if length > 0 => return length.max(max_so_far),
                '$' | ')' => return 0,
                '|' => {
                    max_so_far = max_so_far.max(length);
                    length = 0;
                }
                '^' => {}
                _ => panic!("invalid {ch} in input"),
            }
        }

        length.max(max_so_far)
    }
}

impl Runner for Aoc2018_20 {
    fn name(&self) -> (usize, usize) {
        (2018, 20)
    }

    fn parse(&mut self) {
        self.regex = aoclib::read_lines("input/2018-20.txt")[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut iter = self.regex.chars();
        aoclib::output(self.longest(&mut iter))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twenty_three_doors() {
        let aoc = Aoc2018_20::default();
        let mut iter = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$".chars();
        assert_eq!(23, aoc.longest(&mut iter));
    }

    #[test]
    fn thirty_one_doors() {
        let aoc = Aoc2018_20::default();
        let mut iter = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".chars();
        assert_eq!(31, aoc.longest(&mut iter));
    }

    #[test]
    fn eighteen_doors() {
        let aoc = Aoc2018_20::default();
        let mut iter = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".chars();
        assert_eq!(18, aoc.longest(&mut iter));
    }

    #[test]
    fn ten_doors() {
        let aoc = Aoc2018_20::default();
        let mut iter = "^ENWWW(NEEE|SSE(EE|N))$".chars();
        assert_eq!(10, aoc.longest(&mut iter));
    }
}
