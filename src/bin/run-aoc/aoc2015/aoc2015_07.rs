use std::collections::HashMap;

use aoclib::read_lines;

use crate::Runner;

#[derive(Debug)]
enum Command {
    Num(u16),
    Var(String),
    And(String, String),
    Or(String, String),
    Lshift(String, u16),
    Rshift(String, u16),
    Not(String),
}

pub struct Aoc2015_07 {
    commands: HashMap<String, Command>,
    wire: HashMap<String, u16>,
}

impl Aoc2015_07 {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        for l in read_lines("input/2015-07.txt") {
            let (a, b) = l.split_once(" -> ").unwrap();
            let com = a.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();

            let b = b.to_string();
            if com[0].chars().next().unwrap().is_numeric() {
                commands.insert(b, Command::Num(com[0].parse::<u16>().unwrap()));
            } else if com.len() == 1 {
                commands.insert(b, Command::Var(com[0].clone()));
            } else if com[0] == "NOT" {
                commands.insert(b, Command::Not(com[1].clone()));
            } else {
                match com[1].as_str() {
                    "RSHIFT" => commands.insert(
                        b,
                        Command::Rshift(com[0].clone(), com[2].parse::<u16>().unwrap()),
                    ),
                    "LSHIFT" => commands.insert(
                        b,
                        Command::Lshift(com[0].clone(), com[2].parse::<u16>().unwrap()),
                    ),
                    "AND" => commands.insert(b, Command::And(com[0].clone(), com[2].clone())),
                    "OR" => commands.insert(b, Command::Or(com[0].clone(), com[2].clone())),
                    _ => panic!("input corrupted"),
                };
            }
        }

        println!("commands = {}", commands.len());

        Self {
            commands,
            wire: HashMap::new(),
        }
    }

    pub fn get_value(&mut self, var: &String) -> u16 {
        println!("get {var}");
        if let Some(value) = self.wire.get(var) {
            return *value;
        }

        match self.commands.get(var).unwrap() {
            Command::Num(num) => {
                self.wire.insert(var.clone(), *num);
            }
            Command::Var(name) => {
                let value = self.get_value(&name.clone());
                self.wire.insert(var.clone(), value);
            }
            Command::And(left, right) => {
                let left = left.clone();
                let right = right.clone();

                let left = self.get_value(&left);
                let right = self.get_value(&right);
                self.wire.insert(var.clone(), left & right);
            }
            Command::Or(left, right) => {
                let left = left.clone();
                let right = right.clone();

                let left = self.get_value(&left);
                let right = self.get_value(&right);
                self.wire.insert(var.clone(), left | right);
            }
            Command::Lshift(left, amt) => {
                let left = left.clone();
                let amt = *amt;
                let left = self.get_value(&left);
                self.wire.insert(var.clone(), left << amt);
            }
            Command::Rshift(right, amt) => {
                let right = right.clone();
                let amt = *amt;
                let right = self.get_value(&right);
                self.wire.insert(var.clone(), right >> amt);
            }
            Command::Not(name) => {
                let value = self.get_value(&name.clone());
                self.wire.insert(var.clone(), !value);
            }
        }

        *self.wire.get(var).unwrap()
    }
}

impl Runner for Aoc2015_07 {
    fn name(&self) -> (usize, usize) {
        (2015, 7)
    }

    fn part1(&mut self) -> Vec<String> {
        vec![format!("{}", self.get_value(&"a".to_string()))]
    }

    fn part2(&mut self) -> Vec<String> {
        vec!["unsolved".to_string()]
    }
}
