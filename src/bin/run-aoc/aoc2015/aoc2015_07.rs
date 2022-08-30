use std::cell::RefCell;
use std::collections::HashMap;

use aoclib::read_lines;

use crate::Runner;

#[derive(Debug, PartialEq)]
enum Term {
    Var(String),
    Num(u16),
}

impl From<String> for Term {
    fn from(s: String) -> Self {
        if let Ok(num) = s.parse::<u16>() {
            Term::Num(num)
        } else {
            Term::Var(s)
        }
    }
}

impl From<&String> for Term {
    fn from(s: &String) -> Self {
        if let Ok(num) = s.parse::<u16>() {
            Term::Num(num)
        } else {
            Term::Var(s.clone())
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Num(u16),
    Var(String),
    And(Term, Term),
    Or(Term, Term),
    Lshift(String, u16),
    Rshift(String, u16),
    Not(String),
}

pub struct Aoc2015_07 {
    commands: HashMap<String, Command>,
    wire: RefCell<HashMap<String, u16>>,
    part_1_answer: u16,
}

impl Aoc2015_07 {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            wire: RefCell::new(HashMap::new()),
            part_1_answer: 0,
        }
    }

    #[cfg(test)]
    fn new_for_test(commands: HashMap<String, Command>) -> Self {
        Self {
            commands,
            wire: RefCell::new(HashMap::new()),
            part_1_answer: 0,
        }
    }

    fn load(lines: &[String]) -> HashMap<String, Command> {
        let mut commands = HashMap::new();
        for l in lines {
            let (a, b) = l.split_once(" -> ").unwrap();
            let com = a.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();

            let command = if com.len() == 1 && com[0].chars().next().unwrap().is_numeric() {
                Command::Num(com[0].parse::<u16>().unwrap())
            } else if com.len() == 1 {
                Command::Var(com[0].clone())
            } else if com[0] == "NOT" {
                Command::Not(com[1].clone())
            } else {
                match com[1].as_str() {
                    "RSHIFT" => Command::Rshift(com[0].clone(), com[2].parse::<u16>().unwrap()),
                    "LSHIFT" => Command::Lshift(com[0].clone(), com[2].parse::<u16>().unwrap()),
                    "AND" => Command::And(com[0].clone().into(), com[2].clone().into()),
                    "OR" => Command::Or(com[0].clone().into(), com[2].clone().into()),
                    _ => panic!("input corrupted"),
                }
            };

            commands.insert(b.to_string(), command);
        }

        commands
    }

    fn get_value(&self, var: &Term) -> u16 {
        let var = match var {
            Term::Num(n) => return *n,
            Term::Var(s) => s,
        };

        if let Some(value) = self.wire.borrow().get(var) {
            return *value;
        }

        let command = self.commands.get(var).unwrap();
        let value = match command {
            Command::Num(num) => *num,
            Command::Var(name) => self.get_value(&name.into()),
            Command::And(left, right) => {
                let left = self.get_value(left);
                let right = self.get_value(right);
                left & right
            }
            Command::Or(left, right) => {
                let left = self.get_value(left);
                let right = self.get_value(right);
                left | right
            }
            Command::Lshift(left, amt) => {
                let left = self.get_value(&left.into());
                left << *amt
            }
            Command::Rshift(right, amt) => {
                let right = self.get_value(&right.into());
                right >> *amt
            }
            Command::Not(name) => {
                let value = self.get_value(&name.into());
                !value
            }
        };

        self.wire.borrow_mut().insert(var.clone(), value);

        value
    }

    fn set_b(&self, val: u16) {
        self.wire.borrow_mut().clear();
        self.wire.borrow_mut().insert("b".to_string(), val);
    }
}

impl Runner for Aoc2015_07 {
    fn parse(&mut self) {
        self.commands = Self::load(&read_lines("input/2015-07.txt"));
    }

    fn name(&self) -> (usize, usize) {
        (2015, 7)
    }

    fn part1(&mut self) -> Vec<String> {
        self.part_1_answer = self.get_value(&"a".to_string().into());
        crate::output(self.part_1_answer)
    }

    fn part2(&mut self) -> Vec<String> {
        self.set_b(self.part_1_answer);
        crate::output(self.get_value(&"a".to_string().into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_number() {
        let commands = Aoc2015_07::load(&["123 -> x".to_string()]);
        assert_eq!(commands.get(&"x".to_string()).unwrap(), &Command::Num(123));
    }

    #[test]
    fn can_parse_and() {
        let commands = Aoc2015_07::load(&["x AND y -> z".to_string()]);
        assert_eq!(
            commands.get(&"z".to_string()).unwrap(),
            &Command::And("x".to_string().into(), "y".to_string().into())
        );
    }

    #[test]
    fn can_parse_numeric_and() {
        let commands = Aoc2015_07::load(&["12 AND y -> z".to_string()]);
        assert_eq!(
            commands.get(&"z".to_string()).unwrap(),
            &Command::And("12".to_string().into(), "y".to_string().into())
        );
    }

    #[test]
    fn can_parse_lshift() {
        let commands = Aoc2015_07::load(&["p LSHIFT 2 -> q".to_string()]);
        assert_eq!(
            commands.get(&"q".to_string()).unwrap(),
            &Command::Lshift("p".to_string(), 2),
        );
    }

    #[test]
    fn can_parse_rshift() {
        let commands = Aoc2015_07::load(&["p RSHIFT 2 -> q".to_string()]);
        assert_eq!(
            commands.get(&"q".to_string()).unwrap(),
            &Command::Rshift("p".to_string(), 2),
        );
    }

    #[test]
    fn can_parse_not() {
        let commands = Aoc2015_07::load(&["NOT e -> f".to_string()]);
        assert_eq!(
            commands.get(&"f".to_string()).unwrap(),
            &Command::Not("e".to_string()),
        );
    }

    #[test]
    fn can_parse_assignment() {
        let commands = Aoc2015_07::load(&["e -> f".to_string()]);
        assert_eq!(
            commands.get(&"f".to_string()).unwrap(),
            &Command::Var("e".to_string()),
        );
    }

    #[test]
    fn can_parse_or() {
        let commands = Aoc2015_07::load(&["x OR y -> z".to_string()]);
        assert_eq!(
            commands.get(&"z".to_string()).unwrap(),
            &Command::Or("x".to_string().into(), "y".to_string().into())
        );
    }

    #[test]
    fn can_run_number() {
        let commands = Aoc2015_07::load(&["123 -> x".to_string()]);
        let aoc = Aoc2015_07::new_for_test(commands);
        assert_eq!(aoc.get_value(&"x".to_string().into()), 123);
    }

    #[test]
    fn can_run_and() {
        let commands = Aoc2015_07::load(&[
            "12 -> x".to_string(),
            "13 -> y".to_string(),
            "x AND y -> z".to_string(),
        ]);
        let aoc = Aoc2015_07::new_for_test(commands);
        assert_eq!(aoc.get_value(&"z".to_string().into()), 12);
    }

    #[test]
    fn can_and_a_number() {
        let commands = Aoc2015_07::load(&["12 -> y".to_string(), "13 AND y -> z".to_string()]);
        let aoc = Aoc2015_07::new_for_test(commands);
        assert_eq!(aoc.get_value(&"z".to_string().into()), 12);
    }

    #[test]
    fn can_run_or() {
        let commands = Aoc2015_07::load(&[
            "12 -> x".to_string(),
            "13 -> y".to_string(),
            "x OR y -> z".to_string(),
        ]);
        let aoc = Aoc2015_07::new_for_test(commands);
        assert_eq!(aoc.get_value(&"z".to_string().into()), 13);
    }

    #[test]
    fn can_run_lshift() {
        let commands = Aoc2015_07::load(&["12 -> x".to_string(), "x LSHIFT 1 -> z".to_string()]);
        let aoc = Aoc2015_07::new_for_test(commands);
        assert_eq!(aoc.get_value(&"z".to_string().into()), 24);
    }

    #[test]
    fn can_run_rshift() {
        let commands = Aoc2015_07::load(&["12 -> x".to_string(), "x RSHIFT 1 -> z".to_string()]);
        let aoc = Aoc2015_07::new_for_test(commands);
        assert_eq!(aoc.get_value(&"z".to_string().into()), 6);
    }

    #[test]
    fn can_run_not() {
        let commands = Aoc2015_07::load(&["12 -> x".to_string(), "NOT x -> z".to_string()]);
        let aoc = Aoc2015_07::new_for_test(commands);
        assert_eq!(
            aoc.get_value(&"z".to_string().into()),
            0b1111_1111_1111_0011
        );
    }

    #[test]
    fn can_run_assignment() {
        let commands = Aoc2015_07::load(&["12 -> x".to_string(), "x -> z".to_string()]);
        let aoc = Aoc2015_07::new_for_test(commands);
        assert_eq!(aoc.get_value(&"z".to_string().into()), 12);
    }
}
