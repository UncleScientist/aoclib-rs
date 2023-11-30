use std::collections::HashMap;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_19 {
    grid: Grid,
}

impl Aoc2017_19 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_19 {
    fn name(&self) -> (usize, usize) {
        (2017, 19)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-19.txt");

        self.grid.width = lines[0].len() as i64;
        self.grid.height = lines.len() as i64;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    self.grid.map.insert((row as i64, col as i64), ch);
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        self.grid.go_to_start();
        while self.grid.step() {}
        aoclib::output(self.grid.seen.iter().collect::<String>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.grid.steps)
    }
}

#[derive(Default)]
struct Grid {
    map: HashMap<(i64, i64), char>,
    packet_pos: (i64, i64),
    cur_dir: (i64, i64),
    seen: Vec<char>,
    width: i64,
    height: i64,
    turn: HashMap<(i64, i64), Vec<(i64, i64)>>,
    steps: usize,
}

impl Grid {
    fn go_to_start(&mut self) {
        for col in 0..self.width {
            if self.map.contains_key(&(0, col)) {
                self.packet_pos = (0, col);
                break;
            }
        }
        self.cur_dir = (1, 0);
        self.turn.insert((1, 0), vec![(0, 1), (0, -1)]);
        self.turn.insert((0, 1), vec![(1, 0), (-1, 0)]);
        self.turn.insert((-1, 0), vec![(0, 1), (0, -1)]);
        self.turn.insert((0, -1), vec![(1, 0), (-1, 0)]);
        self.steps = 1;
    }

    fn step(&mut self) -> bool {
        let next_pos = (
            self.packet_pos.0 + self.cur_dir.0,
            self.packet_pos.1 + self.cur_dir.1,
        );
        if let Some(next_char) = self.map.get(&next_pos) {
            // println!("{next_pos:?} -> {next_char}");
            match next_char {
                '+' => {
                    let look = self.turn.get(&self.cur_dir).unwrap();
                    let look_pos = (next_pos.0 + look[0].0, next_pos.1 + look[0].1);
                    if self.map.contains_key(&look_pos) {
                        self.cur_dir = look[0];
                    } else {
                        self.cur_dir = look[1];
                    }
                }
                'A'..='Z' => self.seen.push(*next_char),
                '|' | '-' => {}
                _ => panic!("unexpected character '{next_char}' in input"),
            }
            self.packet_pos = next_pos;
            self.steps += 1;
            true
        } else {
            false
        }
    }
}
