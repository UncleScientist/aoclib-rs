use std::collections::HashMap;

use crate::Runner;

#[derive(Default, Debug)]
pub struct Aoc2017_25 {
    cur_state: usize,
    diagnostic: usize,
    cur_pos: i64,
    // (cur_state, value on tape) -> (value to write, direction, new_state)
    states: HashMap<(usize, u8), (u8, Move, usize)>,
    tape: HashMap<i64, u8>,
}

impl Aoc2017_25 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_25 {
    fn name(&self) -> (usize, usize) {
        (2017, 25)
    }

    fn parse(&mut self) {
        // let lines = aoclib::read_lines("test-input.txt");
        let lines = aoclib::read_lines("input/2017-25.txt");
        self.cur_state = (extract_final_char(&lines[0], '.') as u8 - b'A') as usize;
        self.diagnostic = lines[1]
            .split_ascii_whitespace()
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();

        for i in (2..lines.len()).step_by(9) {
            let cur_state = (extract_final_char(&lines[i], ':') as u8 - b'A') as usize;
            for j in [i + 1, i + 5] {
                let step = parse_step(&lines[j], &lines[j + 1], &lines[j + 2], &lines[j + 3]);
                self.states
                    .insert((cur_state, step.0), (step.1, step.2, step.3));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        for _ in 0..self.diagnostic {
            let cur_val = self.tape.entry(self.cur_pos).or_insert(0);
            let action = self.states.get(&(self.cur_state, *cur_val)).unwrap();
            *cur_val = action.0;
            if action.1 == Move::Left {
                self.cur_pos -= 1;
            } else {
                self.cur_pos += 1;
            }
            self.cur_state = action.2;
        }

        crate::output(self.tape.values().filter(|&x| *x == 1).count())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("All done!")
    }
}

#[derive(Debug, Default, PartialEq)]
enum Move {
    #[default]
    Left,
    Right,
}

impl Move {
    fn parse(s: &str) -> Self {
        match s {
            "right." => Self::Right,
            "left." => Self::Left,
            _ => panic!("can't convert {s} into direction"),
        }
    }
}

fn parse_step(
    cur_value: &str,
    write: &str,
    motion: &str,
    next_state: &str,
) -> (u8, u8, Move, usize) {
    (
        extract_final_char(cur_value, ':') as u8 - b'0',
        extract_final_char(write, '.') as u8 - b'0',
        Move::parse(motion.split_ascii_whitespace().last().unwrap()),
        (extract_final_char(next_state, '.') as u8 - b'A') as usize,
    )
}

fn extract_final_char(s: &str, delim: char) -> char {
    s.split_ascii_whitespace()
        .last()
        .unwrap()
        .split_once(delim)
        .unwrap()
        .0
        .chars()
        .next()
        .unwrap()
}
