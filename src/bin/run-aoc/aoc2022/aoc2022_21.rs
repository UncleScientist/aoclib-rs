use std::collections::HashMap;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_21 {
    tree: HashMap<String, Monkey>,
}

impl Aoc2022_21 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_21 {
    fn name(&self) -> (usize, usize) {
        (2022, 21)
    }

    fn parse(&mut self) {
        let _lines = aoclib::read_lines("test-input.txt");
        let lines = aoclib::read_lines("input/2022-21.txt");

        for line in lines {
            let (left, right) = line.split_once(": ").unwrap();

            if let Ok(num) = right.parse::<i64>() {
                self.tree.insert(left.to_string(), Monkey::Value(num));
            } else {
                let expr = right.split_whitespace().collect::<Vec<_>>();
                self.tree.insert(
                    left.to_string(),
                    Monkey::Binary(
                        expr[0].to_string(),
                        Operation::parse(expr[1].chars().next().unwrap()),
                        expr[2].to_string(),
                    ),
                );
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(eval(&"root".to_string(), &self.tree))
    }

    fn part2(&mut self) -> Vec<String> {
        let root = "root".to_string();
        let path = find_human(&root, &self.tree).unwrap();
        let path = path.iter().rev().copied().collect::<Vec<_>>();

        let (left, right) = match self.tree.get(&root).unwrap() {
            Monkey::Value(_) => panic!("root monkey has no data"),
            Monkey::Binary(left, _, right) => (left, right),
        };

        let correct_val = if left == path[1] {
            eval(right, &self.tree)
        } else {
            eval(left, &self.tree)
        };

        crate::output(find_adjustment(&path, 1, &self.tree, correct_val))
    }
}

// left + right = cv
// left - right = cv
// left * right = cv
// left / right = cv

fn find_adjustment(
    path: &Vec<&String>,
    index: usize,
    tree: &HashMap<String, Monkey>,
    cv: i64,
) -> i64 {
    use Operation::*;

    match tree.get(path[index]).unwrap() {
        Monkey::Value(_) => cv,
        Monkey::Binary(l, op, r) => {
            let left = eval(l, tree);
            let right = eval(r, tree);
            let new_cv = if l == path[index + 1] {
                match op {
                    Addition => cv - right,
                    Subtraction => cv + right,
                    Multiplication => cv / right,
                    Division => cv * right,
                }
            } else {
                match op {
                    Addition => cv - left,
                    Subtraction => left - cv,
                    Multiplication => cv / left,
                    Division => left / cv,
                }
            };
            find_adjustment(path, index + 1, tree, new_cv)
        }
    }
}

fn eval(start: &String, tree: &HashMap<String, Monkey>) -> i64 {
    use Operation::*;
    match tree.get(start).unwrap() {
        Monkey::Value(v) => *v,
        Monkey::Binary(l, op, r) => {
            let l = eval(l, tree);
            let r = eval(r, tree);
            match op {
                Addition => l + r,
                Subtraction => l - r,
                Multiplication => l * r,
                Division => l / r,
            }
        }
    }
}

fn find_human<'a>(loc: &'a String, tree: &'a HashMap<String, Monkey>) -> Option<Vec<&'a String>> {
    if loc == "humn" {
        return Some(vec![loc]);
    }

    if let Some(Monkey::Binary(l, _, r)) = tree.get(loc) {
        if let Some(mut vec) = find_human(l, tree) {
            vec.push(loc);
            return Some(vec);
        }
        if let Some(mut vec) = find_human(r, tree) {
            vec.push(loc);
            return Some(vec);
        }
    }

    None
}

#[derive(Debug)]
enum Monkey {
    Value(i64),
    Binary(String, Operation, String),
}

#[derive(Debug)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    fn parse(ch: char) -> Self {
        match ch {
            '+' => Self::Addition,
            '-' => Self::Subtraction,
            '*' => Self::Multiplication,
            '/' => Self::Division,
            _ => panic!("not an operator: '{ch}'"),
        }
    }
}
