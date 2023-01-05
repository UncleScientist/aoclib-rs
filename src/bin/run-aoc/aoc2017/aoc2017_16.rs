use std::collections::VecDeque;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2017_16 {
    moves: Vec<Move>,
}

impl Aoc2017_16 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_16 {
    fn name(&self) -> (usize, usize) {
        (2017, 16)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-16.txt");

        for entry in lines[0].split(',') {
            self.moves.push(match &entry[0..1] {
                "s" => Move::Spin(entry[1..].parse().unwrap()),
                "x" => {
                    let (left, right) = entry[1..].split_once('/').unwrap();
                    Move::Exchange(left.parse().unwrap(), right.parse().unwrap())
                }
                "p" => {
                    let chars = entry.chars().collect::<Vec<_>>();
                    Move::Partner(chars[1], chars[3])
                }
                _ => {
                    panic!("invalid input: '{entry}'")
                }
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let programs: VecDeque<char> = ('a'..='p').collect();

        crate::output(dance(&programs, &self.moves))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn dance(programs: &VecDeque<char>, moves: &[Move]) -> String {
    let mut programs = programs.clone();

    for step in moves {
        match step {
            Move::Spin(amount) => programs.rotate_right(*amount),
            Move::Exchange(a, b) => programs.swap(*a, *b),
            Move::Partner(a, b) => {
                let positions = programs
                    .iter()
                    .enumerate()
                    .filter(|(_, ch)| *ch == a || *ch == b)
                    .map(|(pos, _)| pos)
                    .collect::<Vec<_>>();
                programs.swap(positions[0], positions[1]);
            }
        }
    }

    programs.iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spin() {
        let programs: VecDeque<char> = ('a'..='e').collect();
        let moves = [Move::Spin(3)];
        assert_eq!("cdeab".to_string(), dance(&programs, &moves));
    }

    #[test]
    fn test_spin2() {
        let programs: VecDeque<char> = ('a'..='e').collect();
        let moves = [Move::Spin(1)];
        assert_eq!("eabcd".to_string(), dance(&programs, &moves));
    }

    #[test]
    fn test_exchange() {
        let programs: VecDeque<char> = VecDeque::from(['e', 'a', 'b', 'c', 'd']);
        let moves = [Move::Exchange(3, 4)];
        assert_eq!("eabdc".to_string(), dance(&programs, &moves));
    }

    #[test]
    fn test_partner() {
        let programs: VecDeque<char> = VecDeque::from(['e', 'a', 'b', 'd', 'c']);
        let moves = [Move::Partner('e', 'b')];
        assert_eq!("baedc".to_string(), dance(&programs, &moves));
    }
}
