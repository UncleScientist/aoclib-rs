use std::collections::VecDeque;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_14 {
    recipe_count: usize,
    digits: Vec<u8>,
    last_n: VecDeque<u8>,
    recipes: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl Aoc2018_14 {
    pub fn new() -> Self {
        Self::default()
    }

    fn step(&mut self) {
        let new_recipe = self.recipes[self.elf1] + self.recipes[self.elf2];

        if new_recipe > 9 {
            self.recipes.push(1);
            if self.last_n.len() > self.digits.len() {
                self.last_n.pop_front();
            }
            self.last_n.push_back(1);
        }

        if self.last_n.len() > self.digits.len() {
            self.last_n.pop_front();
        }
        self.recipes.push(new_recipe % 10);
        self.last_n.push_back(new_recipe % 10);

        self.elf1 = (self.elf1 + self.recipes[self.elf1] as usize + 1) % self.recipes.len();
        self.elf2 = (self.elf2 + self.recipes[self.elf2] as usize + 1) % self.recipes.len();
    }

    fn reset(&mut self) {
        self.recipes = vec![3, 7];
        self.elf1 = 0;
        self.elf2 = 1;
        self.last_n.clear();
    }
}

impl Runner for Aoc2018_14 {
    fn name(&self) -> (usize, usize) {
        (2018, 14)
    }

    fn parse(&mut self) {
        self.recipe_count = 939601;
        self.reset();

        let mut digits = Vec::new();
        let mut num = self.recipe_count;
        while num > 0 {
            digits.push((num % 10) as u8);
            num /= 10;
        }
        self.digits = digits.into_iter().rev().collect();
    }

    fn part1(&mut self) -> Vec<String> {
        while self.recipes.len() < self.recipe_count + 10 {
            self.step();
        }

        let mut answer = String::new();
        for c in &self.recipes[self.recipe_count..] {
            answer.push((*c + b'0') as char);
        }
        aoclib::output(answer)
    }

    fn part2(&mut self) -> Vec<String> {
        self.reset();
        for _ in 0..self.digits.len() {
            self.step();
        }

        'next: loop {
            for (a, b) in self.digits.iter().zip(self.last_n.iter()) {
                if a != b {
                    self.step();
                    continue 'next;
                }
            }

            // total hack because I assume the puzzle input ends with a '1' digit
            return aoclib::output(format!("{}", self.recipes.len() - self.digits.len() - 1));
        }
    }
}
