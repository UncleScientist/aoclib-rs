use std::{collections::HashMap, ops::Range, str::FromStr};

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

    fn poss(&self, rules: &[Rule], mut ranges: [Range<usize>; 4]) -> usize {
        let mut total = 0;

        for rule in rules {
            let mut deeper = ranges.clone();
            match rule.condition {
                Condition::LessThan(var, val) => {
                    deeper[var].end = val - 1;
                    ranges[var].start = val;
                }
                Condition::GreaterThan(var, val) => {
                    deeper[var].start = val + 1;
                    ranges[var].end = val
                }
                Condition::True => {}
            }

            match &rule.action {
                Action::Accept => total += deeper.iter().map(|r| r.len() + 1).product::<usize>(),
                Action::Reject => {}
                Action::GoTo(workflow) => {
                    let wf = self.workflows.get(workflow).unwrap();
                    total += self.poss(wf, deeper);
                }
            }
        }
        total
    }
}

impl Runner for Aoc2023_19 {
    fn name(&self) -> (usize, usize) {
        (2023, 19)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-19.txt");
        //  let lines = aoclib::read_lines("test-input");

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
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;

        'next_part: for part in &self.parts {
            let mut cur_wf = "in".to_string();
            'next_workflow: loop {
                for rule in self.workflows.get(&cur_wf).unwrap().iter() {
                    let pass = match rule.condition {
                        Condition::LessThan(var, val) => part[var] < val,
                        Condition::GreaterThan(var, val) => part[var] > val,
                        Condition::True => true,
                    };
                    if pass {
                        match &rule.action {
                            Action::Accept => total += part.iter().sum::<usize>(),
                            Action::Reject => {}
                            Action::GoTo(wf) => {
                                cur_wf = wf.clone();
                                continue 'next_workflow;
                            }
                        }
                        continue 'next_part;
                    }
                }
            }
        }

        aoclib::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let cur_wf = self.workflows.get("in").unwrap();
        let ranges = [1..4000usize, 1..4000usize, 1..4000usize, 1..4000usize];

        aoclib::output(self.poss(cur_wf, ranges))
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
