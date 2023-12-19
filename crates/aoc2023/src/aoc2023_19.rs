use std::{collections::HashMap, str::FromStr};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_19 {
    parts: Vec<Part>,
    workflows: HashMap<String, Vec<Rule>>,
}

impl Aoc2023_19 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_19 {
    fn name(&self) -> (usize, usize) {
        (2023, 19)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("test-input");

        for line in lines {
            if line.starts_with('{') {
                // parse part
                let line = line.strip_suffix('}').unwrap();
                let line = line.strip_prefix('{').unwrap();
                let mut part = [0usize; 4];
                for section in line.split(',') {
                    let (ch, num) = section.split_once('=').unwrap();
                    match ch {
                        "x" => part[0] = num.parse().unwrap(),
                        "m" => part[1] = num.parse().unwrap(),
                        "a" => part[2] = num.parse().unwrap(),
                        "s" => part[3] = num.parse().unwrap(),
                        _ => panic!("invalid part: {line}"),
                    }
                }
                self.parts.push(part);
            } else {
                let (name, ruleset) = line.split_once('{').unwrap();
                let ruleset = ruleset.strip_suffix('}').unwrap();
                let mut rules: Vec<Rule> = Vec::new();
                for rule in ruleset.split(',') {
                    rules.push(rule.parse().unwrap());
                }

                self.workflows.insert(name.to_string(), rules);
            }
        }
        println!("{:?}", self.workflows);
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

type Part = [usize; 4];

#[derive(Debug)]
struct Rule {
    condition: Condition,
    action: Action,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cond, act)) = s.split_once(':') {
            Ok(Self {
                condition: cond.parse().unwrap(),
                action: act.parse().unwrap(),
            })
        } else {
            Ok(Self {
                condition: Condition::True,
                action: s.parse().unwrap(),
            })
        }
    }
}

#[derive(Debug)]
enum Condition {
    LessThan(usize, usize),
    GreaterThan(usize, usize),
    True,
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[2..].parse::<usize>().unwrap();
        let index = match &s[..1] {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => return Err(format!("Invalid condition: {s}")),
        };
        match &s[1..2] {
            "<" => Ok(Self::LessThan(index, value)),
            ">" => Ok(Self::GreaterThan(index, value)),
            _ => Err(format!("invalid condition: {s}")),
        }
    }
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    GoTo(String),
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::GoTo(s.to_string())),
        }
    }
}
