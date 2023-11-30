use std::cmp::Ordering;
use std::str::Chars;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2022_13 {
    packets: Vec<Val>,
}

impl Aoc2022_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_13 {
    fn name(&self) -> (usize, usize) {
        (2022, 13)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2022-13.txt") {
            self.packets.push(Val::parse(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut sum = 0;
        for (index, pair) in self.packets.chunks(2).enumerate() {
            if pair[0] < pair[1] {
                sum += index + 1;
            }
        }
        aoclib::output(sum)
    }

    fn part2(&mut self) -> Vec<String> {
        let d2 = Val::parse("[[2]]");
        let d6 = Val::parse("[[6]]");

        let mut list = vec![d2.clone(), d6.clone()];
        list.extend(self.packets.iter().cloned());

        list.sort(); // <- does all the work!

        aoclib::output(
            list.into_iter()
                .enumerate()
                .filter(|(_, p)| *p == d2 || *p == d6)
                .fold(1, |a, b| a * (b.0 + 1)),
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Val {
    Num(i32),
    List(Vec<Self>),
}

impl Val {
    fn parse(s: &str) -> Self {
        let mut c = s.chars();
        if c.next().unwrap() != '[' {
            panic!("bad input");
        }
        Self::parse_into(&mut c)
    }

    fn parse_into(c: &mut Chars) -> Self {
        let mut result = Vec::new();
        let mut num = -1;
        while let Some(ch) = c.next() {
            match ch {
                '[' => result.push(Self::parse_into(c)),
                ',' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                        num = -1;
                    }
                }
                ']' => {
                    if num >= 0 {
                        result.push(Self::Num(num))
                    }
                    return Self::List(result);
                }
                '0'..='9' => {
                    if num == -1 {
                        num = (ch as u8 - b'0') as i32;
                    } else {
                        num = (num * 10) + (ch as u8 - b'0') as i32;
                    }
                }
                _ => panic!("bad char '{ch}'"),
            }
        }
        Self::List(result)
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val::List(left), Val::List(right)) => {
                let mut idx = 0;
                while idx < left.len() && idx < right.len() {
                    match (&left[idx], &right[idx]) {
                        (Val::Num(l), Val::Num(r)) => {
                            if l != r {
                                return l.cmp(r);
                            }
                        }
                        (Val::List(_), Val::Num(r)) => {
                            let check = left[idx].compare(&Val::List(vec![Val::Num(*r)]));
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::Num(l), Val::List(_)) => {
                            let check = Val::List(vec![Val::Num(*l)]).compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::List(_), Val::List(_)) => {
                            let check = left[idx].compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                    }
                    idx += 1;
                }
                left.len().cmp(&right.len())
            }
            _ => panic!("bad input"),
        }
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}
