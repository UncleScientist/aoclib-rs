use std::collections::VecDeque;

use aoclib::Permutations;

use crate::Runner;

pub struct Aoc2016_21 {
    inst: Vec<Inst>,
}

impl Aoc2016_21 {
    pub fn new() -> Self {
        Self { inst: Vec::new() }
    }
}

impl Runner for Aoc2016_21 {
    fn name(&self) -> (usize, usize) {
        (2016, 21)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2016-21.txt");
        for l in lines {
            let word = l.split(' ').collect::<Vec<&str>>();
            self.inst.push(match (word[0], word[1]) {
                ("swap", "position") => {
                    Inst::SwapPos(word[2].parse().unwrap(), word[5].parse().unwrap())
                }
                ("swap", "letter") => Inst::SwapChar(
                    word[2].chars().next().unwrap(),
                    word[5].chars().next().unwrap(),
                ),
                ("rotate", "based") => Inst::RotChar(word[6].chars().next().unwrap()),
                ("rotate", "right") => Inst::RotRight(word[2].parse().unwrap()),
                ("rotate", "left") => Inst::RotLeft(word[2].parse().unwrap()),
                ("reverse", _) => Inst::Reverse(word[2].parse().unwrap(), word[4].parse().unwrap()),
                ("move", _) => Inst::Move(word[2].parse().unwrap(), word[5].parse().unwrap()),
                _ => panic!("Don't know how to handle '{} {}'", word[0], word[1]),
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut result = "abcdefgh".to_string();
        for i in &self.inst {
            result = result.op(i);
        }
        crate::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let vec = "abcdefgh".chars().collect::<Vec<char>>();
        let mut answer = None;
        for p in vec.permutations() {
            let mut result = String::from_iter(&p);
            for i in &self.inst {
                result = result.op(i);
            }
            if result == "fbgdceah" {
                answer = Some(String::from_iter(&p));
                break;
            }
        }
        crate::output(answer.unwrap())
    }
}

#[derive(Debug)]
enum Inst {
    SwapPos(usize, usize),
    SwapChar(char, char),
    RotLeft(usize),
    RotRight(usize),
    RotChar(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

trait Operation {
    fn op(&self, i: &Inst) -> Self;
}

impl Operation for String {
    fn op(&self, i: &Inst) -> String {
        let mut input = self.chars().collect::<VecDeque<char>>();

        match i {
            Inst::SwapPos(a, b) => input.swap(*a, *b),
            Inst::SwapChar(a, b) => {
                let mut loc_a = None;
                let mut loc_b = None;
                for (idx, ch) in input.iter().enumerate() {
                    if ch == a {
                        loc_a = Some(idx);
                    }
                    if ch == b {
                        loc_b = Some(idx);
                    }
                }
                input.swap(loc_a.unwrap(), loc_b.unwrap());
            }
            Inst::Reverse(mut start, mut end) => {
                while start < end {
                    input.swap(start, end);
                    start += 1;
                    end -= 1;
                }
            }
            Inst::RotLeft(count) => input.rotate_left(*count),
            Inst::RotRight(count) => input.rotate_right(*count),
            Inst::Move(from, to) => {
                let ch = input.remove(*from).unwrap();
                input.insert(*to, ch);
            }
            Inst::RotChar(src) => {
                let mut loc = None;
                for (idx, ch) in input.iter().enumerate() {
                    if ch == src {
                        loc = Some(idx);
                        break;
                    }
                }

                let loc = loc.unwrap();
                input.rotate_right((1 + loc + (loc >= 4) as usize) % input.len());
            }
        }

        String::from_iter(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn swap_pos() {
        assert_eq!(
            "ebcda".to_string(),
            "abcde".to_string().op(&Inst::SwapPos(4, 0))
        );
    }

    #[test]
    fn swap_char() {
        assert_eq!(
            "edcba".to_string(),
            "ebcda".to_string().op(&Inst::SwapChar('d', 'b'))
        );
    }

    #[test]
    fn reverse() {
        assert_eq!(
            "abcde".to_string(),
            "edcba".to_string().op(&Inst::Reverse(0, 4))
        );
    }

    #[test]
    fn rotate_left() {
        assert_eq!(
            "bcdea".to_string(),
            "abcde".to_string().op(&Inst::RotLeft(1))
        );
    }

    #[test]
    fn move_position() {
        assert_eq!(
            "bdeac".to_string(),
            "bcdea".to_string().op(&Inst::Move(1, 4))
        );
    }

    #[test]
    fn rotate_based() {
        assert_eq!(
            "decab".to_string(),
            "ecabd".to_string().op(&Inst::RotChar('d'))
        );
    }
}
