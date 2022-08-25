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
}

impl Aoc2015_07 {
    pub fn new() -> Self {
        Self {
            commands: Aoc2015_07::load(&read_lines("input/2015-07.txt")),
            wire: RefCell::new(HashMap::new()),
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
                let left = left.clone();
                let right = right.clone();

                let left = self.get_value(&left);
                let right = self.get_value(&right);
                left & right
            }
            Command::Or(left, right) => {
                let left = left.clone();
                let right = right.clone();

                let left = self.get_value(&left);
                let right = self.get_value(&right);
                left | right
            }
            Command::Lshift(left, amt) => {
                let left = left.clone();
                let amt = *amt;
                let left = self.get_value(&left.into());
                left << amt
            }
            Command::Rshift(right, amt) => {
                let right = right.clone();
                let amt = *amt;
                let right = self.get_value(&right.into());
                right >> amt
            }
            Command::Not(name) => {
                let value = self.get_value(&name.into());
                !value
            }
        };

        self.wire.borrow_mut().insert(var.clone(), value);

        value
    }
}

impl Runner for Aoc2015_07 {
    fn name(&self) -> (usize, usize) {
        (2015, 7)
    }

    fn part1(&mut self) -> Vec<String> {
        vec![format!("{}", self.get_value(&"a".to_string().into()))]
    }

    fn part2(&mut self) -> Vec<String> {
        vec!["unsolved".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_number() {
        let commands = Aoc2015_07::load(&vec!["123 -> x".to_string()]);
        assert_eq!(commands.get(&"x".to_string()).unwrap(), &Command::Num(123));
    }

    #[test]
    fn can_parse_and() {
        let commands = Aoc2015_07::load(&vec!["x AND y -> z".to_string()]);
        assert_eq!(
            commands.get(&"z".to_string()).unwrap(),
            &Command::And("x".to_string().into(), "y".to_string().into())
        );
    }

    #[test]
    fn can_parse_numeric_and() {
        let commands = Aoc2015_07::load(&vec!["12 AND y -> z".to_string()]);
        assert_eq!(
            commands.get(&"z".to_string()).unwrap(),
            &Command::And("12".to_string().into(), "y".to_string().into())
        );
    }

    #[test]
    fn can_parse_lshift() {
        let commands = Aoc2015_07::load(&vec!["p LSHIFT 2 -> q".to_string()]);
        assert_eq!(
            commands.get(&"q".to_string()).unwrap(),
            &Command::Lshift("p".to_string().into(), 2),
        );
    }

    #[test]
    fn can_parse_rshift() {
        let commands = Aoc2015_07::load(&vec!["p RSHIFT 2 -> q".to_string()]);
        assert_eq!(
            commands.get(&"q".to_string()).unwrap(),
            &Command::Rshift("p".to_string().into(), 2),
        );
    }

    #[test]
    fn can_parse_not() {
        let commands = Aoc2015_07::load(&vec!["NOT e -> f".to_string()]);
        assert_eq!(
            commands.get(&"f".to_string()).unwrap(),
            &Command::Not("e".to_string().into()),
        );
    }

    #[test]
    fn can_parse_assignment() {
        let commands = Aoc2015_07::load(&vec!["e -> f".to_string()]);
        assert_eq!(
            commands.get(&"f".to_string()).unwrap(),
            &Command::Var("e".to_string().into()),
        );
    }

    #[test]
    fn can_parse_or() {
        let commands = Aoc2015_07::load(&vec!["x OR y -> z".to_string()]);
        assert_eq!(
            commands.get(&"z".to_string()).unwrap(),
            &Command::Or("x".to_string().into(), "y".to_string().into())
        );
    }

    #[test]
    fn can_run_number() {
        let commands = Aoc2015_07::load(&vec!["123 -> x".to_string()]);
        let aoc = Aoc2015_07 {
            commands,
            wire: RefCell::new(HashMap::new()),
        };
        assert_eq!(aoc.get_value(&"x".to_string().into()), 123);
    }

    #[test]
    fn can_run_and() {
        let commands = Aoc2015_07::load(&vec![
            "12 -> x".to_string(),
            "13 -> y".to_string(),
            "x AND y -> z".to_string(),
        ]);
        let aoc = Aoc2015_07 {
            commands,
            wire: RefCell::new(HashMap::new()),
        };
        assert_eq!(aoc.get_value(&"z".to_string().into()), 12);
    }

    #[test]
    fn can_and_a_number() {
        let commands = Aoc2015_07::load(&vec!["12 -> y".to_string(), "13 AND y -> z".to_string()]);
        let aoc = Aoc2015_07 {
            commands,
            wire: RefCell::new(HashMap::new()),
        };
        assert_eq!(aoc.get_value(&"z".to_string().into()), 12);
    }

    #[test]
    fn can_run_or() {
        let commands = Aoc2015_07::load(&vec![
            "12 -> x".to_string(),
            "13 -> y".to_string(),
            "x OR y -> z".to_string(),
        ]);
        let aoc = Aoc2015_07 {
            commands,
            wire: RefCell::new(HashMap::new()),
        };
        assert_eq!(aoc.get_value(&"z".to_string().into()), 13);
    }

    #[test]
    fn can_run_lshift() {
        let commands =
            Aoc2015_07::load(&vec!["12 -> x".to_string(), "x LSHIFT 1 -> z".to_string()]);
        let aoc = Aoc2015_07 {
            commands,
            wire: RefCell::new(HashMap::new()),
        };
        assert_eq!(aoc.get_value(&"z".to_string().into()), 24);
    }

    #[test]
    fn can_run_rshift() {
        let commands =
            Aoc2015_07::load(&vec!["12 -> x".to_string(), "x RSHIFT 1 -> z".to_string()]);
        let aoc = Aoc2015_07 {
            commands,
            wire: RefCell::new(HashMap::new()),
        };
        assert_eq!(aoc.get_value(&"z".to_string().into()), 6);
    }

    #[test]
    fn can_run_not() {
        let commands = Aoc2015_07::load(&vec!["12 -> x".to_string(), "NOT x -> z".to_string()]);
        let aoc = Aoc2015_07 {
            commands,
            wire: RefCell::new(HashMap::new()),
        };
        assert_eq!(
            aoc.get_value(&"z".to_string().into()),
            0b1111_1111_1111_0011
        );
    }

    #[test]
    fn can_run_assignment() {
        let commands = Aoc2015_07::load(&vec!["12 -> x".to_string(), "x -> z".to_string()]);
        let aoc = Aoc2015_07 {
            commands,
            wire: RefCell::new(HashMap::new()),
        };
        assert_eq!(aoc.get_value(&"z".to_string().into()), 12);
    }
}
