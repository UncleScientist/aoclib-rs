use std::collections::HashSet;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_04 {
    cards: Vec<Card>,
}

impl Aoc2023_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_04 {
    fn name(&self) -> (usize, usize) {
        (2023, 4)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-04.txt");

        for line in lines {
            let (_, nums) = line.split_once(": ").unwrap();
            let (win, chose) = nums.split_once(" | ").unwrap();

            let winning_numbers = win
                .split_whitespace()
                .map(|snum| snum.parse::<i64>().unwrap())
                .collect::<HashSet<_>>();
            let chosen_numbers = chose
                .split_whitespace()
                .map(|snum| snum.parse::<i64>().unwrap())
                .collect::<HashSet<_>>();
            self.cards.push(Card {
                winning_numbers,
                chosen_numbers,
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.cards.iter().map(Card::score).sum::<i64>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

struct Card {
    winning_numbers: HashSet<i64>,
    chosen_numbers: HashSet<i64>,
}

impl Card {
    fn score(&self) -> i64 {
        let count = self
            .winning_numbers
            .intersection(&self.chosen_numbers)
            .count();
        if count > 0 {
            1 << (count - 1)
        } else {
            0
        }
    }
}
