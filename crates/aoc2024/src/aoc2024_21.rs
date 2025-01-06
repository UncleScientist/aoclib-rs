use std::str::FromStr;

use aoclib::{Position64, Runner};

#[derive(Default)]
pub struct Aoc2024_21 {
    keypads: Vec<Keypad>,
}

impl Aoc2024_21 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_21 {
    fn name(&self) -> (usize, usize) {
        (2024, 21)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-21.txt");
        self.keypads = lines.iter().map(|line| line.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let total = self.keypads.iter().map(|kp| kp.score()).sum::<usize>();
        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum KeypadKind {
    Numeric,
    Directional,
}

impl KeypadKind {
    fn start(&self) -> Position64 {
        match self {
            KeypadKind::Numeric => Position64(3, 2),
            KeypadKind::Directional => Position64(0, 2),
        }
    }

    fn blank_spot(&self) -> Position64 {
        match self {
            KeypadKind::Numeric => Position64(3, 0),
            KeypadKind::Directional => Position64(0, 0),
        }
    }

    fn keypad_loc(&self, ch: char) -> Position64 {
        assert_eq!(*self, KeypadKind::Numeric);
        match ch {
            '7' => Position64(0, 0),
            '8' => Position64(0, 1),
            '9' => Position64(0, 2),
            '4' => Position64(1, 0),
            '5' => Position64(1, 1),
            '6' => Position64(1, 2),
            '1' => Position64(2, 0),
            '2' => Position64(2, 1),
            '3' => Position64(2, 2),
            '0' => Position64(3, 1),
            'A' => Position64(3, 2),
            _ => panic!("invalid position '{ch}'"),
        }
    }

    fn direction_loc(&self, motion: &Motion) -> Position64 {
        assert_eq!(*self, KeypadKind::Directional);
        match motion {
            Motion::Up => Position64(0, 1),
            Motion::Down => Position64(1, 1),
            Motion::Left => Position64(1, 0),
            Motion::Right => Position64(1, 2),
            Motion::Enter => Position64(0, 2),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
    Enter,
}

impl Motion {
    fn to_char(&self) -> char {
        match self {
            Motion::Up => '^',
            Motion::Down => 'v',
            Motion::Left => '<',
            Motion::Right => '>',
            Motion::Enter => 'A',
        }
    }
}

fn _motion_to_string(motion: &[Motion]) -> String {
    motion.iter().map(Motion::to_char).collect()
}

#[derive(Debug)]
struct Keypad {
    kind: KeypadKind,
    code: Vec<char>,
}

impl Keypad {
    fn directional() -> Self {
        Self {
            kind: KeypadKind::Directional,
            code: Vec::new(),
        }
    }

    fn get_length(&self) -> usize {
        let directional = Self::directional();
        let mut list = [self.generate(false), self.generate(true)]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        println!("{}", list.len());

        for _ in 0..2 {
            let mut next = Vec::new();
            for item in list {
                next.extend(
                    [
                        directional.generate_directional_motion(&item, false),
                        directional.generate_directional_motion(&item, true),
                    ]
                    .into_iter()
                    .flatten(),
                );
            }
            println!("next list {}", next.len());
            list = next;
        }

        list.iter().map(|item| item.len()).min().unwrap()
    }

    fn value(&self) -> usize {
        assert_eq!(self.kind, KeypadKind::Numeric);
        (self.code[0] as u8 - b'0') as usize * 100
            + (self.code[1] as u8 - b'0') as usize * 10
            + (self.code[2] as u8 - b'0') as usize
    }

    fn score(&self) -> usize {
        self.get_length() * self.value()
    }

    fn generate(&self, lr_first: bool) -> Option<Vec<Motion>> {
        let blank = self.kind.blank_spot();
        let mut cur = self.kind.start();

        let mut result = Vec::new();
        for c in &self.code {
            let dest = self.kind.keypad_loc(*c);

            let (dr, dc) = (dest.0 - cur.0, dest.1 - cur.1);
            let (sr, sc) = (dr.signum(), dc.signum());

            if lr_first {
                result.extend(Self::left_right(&mut cur, &dest, sc, &blank)?);
                result.extend(Self::up_down(&mut cur, &dest, sr, &blank)?);
            } else {
                result.extend(Self::up_down(&mut cur, &dest, sr, &blank)?);
                result.extend(Self::left_right(&mut cur, &dest, sc, &blank)?);
            }

            result.push(Motion::Enter);

            cur = dest;
        }

        Some(result)
    }

    fn generate_directional_motion(&self, seq: &[Motion], lr_first: bool) -> Option<Vec<Motion>> {
        let blank = self.kind.blank_spot();
        let mut cur = self.kind.start();

        let mut result = Vec::new();
        for m in seq {
            let dest = self.kind.direction_loc(m);
            let (dr, dc) = (dest.0 - cur.0, dest.1 - cur.1);
            let (sr, sc) = (dr.signum(), dc.signum());
            println!("{lr_first} - {cur:?} -> {dest:?}");
            for lr_first in [false, true] {
                let (seq1, seq2) = if lr_first {
                    (
                        Self::left_right(&mut cur, &dest, sc, &blank),
                        Self::up_down(&mut cur, &dest, sr, &blank),
                    )
                } else {
                    (
                        Self::up_down(&mut cur, &dest, sr, &blank),
                        Self::left_right(&mut cur, &dest, sc, &blank),
                    )
                };
                if seq1.is_none() || seq2.is_none() {
                    continue;
                }
                result.extend(seq1.unwrap());
                result.extend(seq2.unwrap());
            }

            result.push(Motion::Enter);

            cur = dest;
        }

        println!(" found path: {result:?}");
        Some(result)
    }

    fn up_down(
        cur: &mut Position64,
        dest: &Position64,
        delta: i64,
        blank: &Position64,
    ) -> Option<Vec<Motion>> {
        let mut result = Vec::new();
        while cur.0 != dest.0 {
            if delta < 0 {
                result.push(Motion::Up);
            } else {
                result.push(Motion::Down);
            }
            cur.0 += delta;
            println!("  ud step {cur:?}");
            if cur == blank {
                println!("up/down reached {cur:?}");
                return None;
            }
        }
        Some(result)
    }

    fn left_right(
        cur: &mut Position64,
        dest: &Position64,
        delta: i64,
        blank: &Position64,
    ) -> Option<Vec<Motion>> {
        let mut result = Vec::new();
        while cur.1 != dest.1 {
            if delta < 0 {
                result.push(Motion::Left);
            } else {
                result.push(Motion::Right);
            }
            cur.1 += delta;
            println!("  lr step {cur:?}");
            if cur == blank {
                println!("left/right reached {cur:?}");
                return None;
            }
        }
        Some(result)
    }
}

impl FromStr for Keypad {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.chars().collect::<Vec<_>>();
        Ok(Keypad {
            kind: KeypadKind::Numeric,
            code,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_seq() {
        let keypad: Keypad = "029A".parse().unwrap();
        let seq = keypad.generate(false).unwrap();
        assert_eq!("<A^A^^>AvvvA".to_string(), _motion_to_string(&seq));
    }

    #[test]
    fn test_next_seq() {
        let keypad: Keypad = "029A".parse().unwrap();
        let directional = Keypad::directional();

        let seq = keypad.generate(false).unwrap();
        let seq2 = directional
            .generate_directional_motion(&seq, false)
            .unwrap();
        assert_eq!(
            "v<<A>>^A<A>A<AAv>A^Av<AAA>^A".to_string(),
            _motion_to_string(&seq2)
        );
    }

    #[test]
    fn test_seq_len() {
        let keypad: Keypad = "029A".parse().unwrap();
        assert_eq!(68, keypad.get_length());
    }

    #[test]
    fn test_value() {
        let keypad: Keypad = "029A".parse().unwrap();
        assert_eq!(29, keypad.value());
    }

    #[test]
    #[ignore]
    fn test_379_first_seq() {
        let keypad: Keypad = "179A".parse().unwrap();
        let directional = Keypad::directional();
        let seq = keypad.generate(false).unwrap();
        println!("seq: {}", _motion_to_string(&seq));
        let seq = directional
            .generate_directional_motion(&seq, false)
            .unwrap();
        println!("seq: {}", _motion_to_string(&seq));
        let seq = directional
            .generate_directional_motion(&seq, false)
            .unwrap();
        assert_eq!("", _motion_to_string(&seq));
    }

    #[test]
    fn test_score() {
        let keypads = ["029A", "980A", "179A", "456A", "379A"];
        let total = keypads
            .iter()
            .map(|kp| kp.parse::<Keypad>().unwrap().score())
            .sum::<usize>();
        assert_eq!(126384, total);
    }
}
