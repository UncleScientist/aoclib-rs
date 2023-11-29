use std::collections::VecDeque;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_09 {
    player_count: i64,
    marble_count: i64,
}

impl Aoc2018_09 {
    pub fn new() -> Self {
        Self::default()
    }

    fn solve_for(&self, marbles: i64) -> i64 {
        let mut score = vec![0i64; self.player_count as usize];
        let mut circle = VecDeque::new();

        circle.push_back(0i64);

        let mut current_player = 0;
        for m in 1..=marbles {
            if m % 23 == 0 {
                score[current_player] += m;
                for _ in 0..6 {
                    let tail = circle.pop_back().unwrap();
                    circle.push_front(tail);
                }
                score[current_player] += circle.pop_back().unwrap();
            } else {
                let head = circle.pop_front().unwrap();
                circle.push_back(head);
                let head = circle.pop_front().unwrap();
                circle.push_back(head);
                circle.push_front(m);
            }

            current_player = (current_player + 1) % self.player_count as usize;
        }

        *score.iter().max().unwrap()
    }
}

impl Runner for Aoc2018_09 {
    fn name(&self) -> (usize, usize) {
        (2018, 9)
    }

    fn parse(&mut self) {
        // self.player_count = 9;
        // self.marble_count = 25;
        let lines = aoclib::read_lines("input/2018-09.txt")
            .first()
            .unwrap()
            .clone();
        let words = lines.split_whitespace().collect::<Vec<_>>();
        self.player_count = words[0].parse().unwrap();
        self.marble_count = words[6].parse().unwrap();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.solve_for(self.marble_count))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.solve_for(self.marble_count * 100))
    }
}
