use std::{collections::HashSet, str::FromStr};

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
        let total = self.keypads.iter().map(|kp| kp.score(2)).sum::<usize>();
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

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
    Enter,
}

impl Motion {
    fn _to_char(&self) -> char {
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
    motion.iter().map(Motion::_to_char).collect()
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

    fn get_length(&self, depth: usize) -> usize {
        let directional = Self::directional();
        let list = self.find_keypad_paths();

        let mut shortest = usize::MAX;

        for path in list {
            let mut cur = directional.kind.start();
            let mut total = 0;
            for entry in &path {
                let dest = directional.kind.direction_loc(entry);
                total += directional.find_shortest(depth, &cur, &dest);
                cur = dest;
            }
            shortest = shortest.min(total);
        }

        shortest
    }

    fn value(&self) -> usize {
        assert_eq!(self.kind, KeypadKind::Numeric);
        (self.code[0] as u8 - b'0') as usize * 100
            + (self.code[1] as u8 - b'0') as usize * 10
            + (self.code[2] as u8 - b'0') as usize
    }

    fn score(&self, depth: usize) -> usize {
        self.get_length(depth) * self.value()
    }

    fn find_keypad_paths(&self) -> HashSet<Vec<Motion>> {
        let mut cur = self.kind.start();
        let dest = self.kind.keypad_loc(self.code[0]);

        let mut opts = self.get_options(&cur, &dest);

        cur = dest;
        for keypad_char in self.code.iter().skip(1) {
            let dest = self.kind.keypad_loc(*keypad_char);
            let next_opts = self.get_options(&cur, &dest);
            let mut step = HashSet::new();

            for opt in opts {
                for next in &next_opts {
                    step.insert(opt.iter().copied().chain(next.iter().copied()).collect());
                }
            }

            opts = step;
            cur = dest;
        }

        opts
    }

    fn find_shortest(&self, depth: usize, src: &Position64, dest: &Position64) -> usize {
        let opts = self.get_options(src, dest);

        if depth == 1 {
            return opts.iter().map(|entry| entry.len()).min().unwrap();
        }

        let mut min = usize::MAX;
        for opt in opts {
            let mut cur = self.kind.start();
            let mut total = 0;
            for m in opt {
                let dest = self.kind.direction_loc(&m);
                total += self.find_shortest(depth - 1, &cur, &dest);
                cur = dest;
            }
            min = min.min(total);
        }

        min
    }

    fn get_options(&self, src: &Position64, dest: &Position64) -> HashSet<Vec<Motion>> {
        let blank = self.kind.blank_spot();
        let mut result = HashSet::new();

        let (dr, dc) = (dest.0 - src.0, dest.1 - src.1);
        let (sr, sc) = (dr.signum(), dc.signum());

        let mut cur = *src;
        let lr = Self::left_right(&mut cur, dest, sc, &blank);
        let ud = Self::up_down(&mut cur, dest, sr, &blank);
        if let (Some(mut lr), Some(ud)) = (lr, ud) {
            lr.extend(ud);
            lr.push(Motion::Enter);
            result.insert(lr);
        }

        let mut cur = *src;
        let ud = Self::up_down(&mut cur, dest, sr, &blank);
        let lr = Self::left_right(&mut cur, dest, sc, &blank);
        if let (Some(lr), Some(mut ud)) = (lr, ud) {
            ud.extend(lr);
            ud.push(Motion::Enter);
            result.insert(ud);
        }

        result
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
            if cur == blank {
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
            if cur == blank {
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
        let seq = keypad.find_keypad_paths();
        let seq = seq.iter().next().unwrap();
        assert_eq!("<A^A^^>AvvvA".to_string(), _motion_to_string(seq));
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

    /*
    #[test]
    #[ignore]
    fn test_379_first_seq() {
        let keypad: Keypad = "179A".parse().unwrap();
        let directional = Keypad::directional();
        let seq = keypad.generate();
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
    */

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
