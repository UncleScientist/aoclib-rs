use std::collections::VecDeque;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_05 {
    ship: Vec<VecDeque<char>>,
    instructions: Vec<Move>,
}

impl Aoc2022_05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_05 {
    fn name(&self) -> (usize, usize) {
        (2022, 5)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2022-05.txt") {
            let words = line.split(' ').collect::<Vec<&str>>();
            if words[0] == "move" {
                let amount = words[1].parse().unwrap();
                let from = words[3].parse::<usize>().unwrap() - 1;
                let to = words[5].parse::<usize>().unwrap() - 1;
                self.instructions.push(Move { amount, from, to });
            } else if line.contains('[') {
                for pair in line
                    .chars()
                    .enumerate()
                    .filter(|(i, c)| *c != ' ' && (i + 3) % 4 == 0)
                {
                    let stack = (pair.0 - 1) / 4;
                    while self.ship.len() < stack + 1 {
                        self.ship.push(VecDeque::new());
                    }
                    self.ship[stack].push_front(pair.1)
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut ship = self.ship.clone();

        for i in &self.instructions {
            for _ in 0..i.amount {
                let ship_crate = ship[i.from].pop_back().unwrap();
                ship[i.to].push_back(ship_crate);
            }
        }

        crate::output(stack_tops(&ship))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut ship = self.ship.clone();

        for i in &self.instructions {
            let split_point = ship[i.from].len() - i.amount;
            let removed = ship[i.from].split_off(split_point);
            ship[i.to].extend(removed);
        }

        crate::output(stack_tops(&ship))
    }
}

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn stack_tops(ship: &Vec<VecDeque<char>>) -> String {
    let mut answer = "".to_string();
    for stack in ship {
        answer.push(*stack.back().unwrap());
    }
    answer
}
