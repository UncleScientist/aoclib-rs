use std::collections::HashMap;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_04 {
    guards: HashMap<usize, Guard>,
}

impl Aoc2018_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_04 {
    fn name(&self) -> (usize, usize) {
        (2018, 4)
    }

    fn parse(&mut self) {
        let mut log = aoclib::read_lines("input/2018-04.txt");
        log.sort();

        let mut guard_id = 0;
        let mut sleep_time = 0;

        for line in &log {
            match Log::parse(line) {
                Log::GuardID(g) => guard_id = g,
                Log::Sleep(s) => sleep_time = s,
                Log::Wake(wake_time) => {
                    let guard = self.guards.entry(guard_id).or_insert(Guard::new());
                    for t in sleep_time..wake_time {
                        guard.minute[t] += 1;
                    }
                    guard.total += wake_time - sleep_time;
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let (guard_id, guard_data) = self.guards.iter().max_by_key(|g| g.1.total).unwrap();
        crate::output(*guard_id * guard_data.max_minute())
    }

    fn part2(&mut self) -> Vec<String> {
        let (guard_id, guard_data) = self.guards.iter().max_by_key(|g| g.1.max_time()).unwrap();
        crate::output(*guard_id * guard_data.max_minute())
    }
}

#[derive(Debug)]
struct Guard {
    total: usize,
    minute: [usize; 60],
}

impl Guard {
    fn new() -> Self {
        Self {
            total: 0,
            minute: [0usize; 60],
        }
    }

    fn max_time(&self) -> usize {
        *self.minute.iter().max().unwrap()
    }

    fn max_minute(&self) -> usize {
        self.minute
            .iter()
            .enumerate()
            .max_by_key(|m| m.1)
            .unwrap()
            .0
    }
}

enum Log {
    GuardID(usize),
    Sleep(usize),
    Wake(usize),
}

impl Log {
    fn parse(s: &str) -> Self {
        let split = s.split(' ').collect::<Vec<_>>();
        let minute = split[1][3..5].parse().unwrap();
        match split[2] {
            "Guard" => Self::GuardID(split[3][1..].parse().unwrap()),
            "falls" => Self::Sleep(minute),
            "wakes" => Self::Wake(minute),
            _ => panic!("invalid input"),
        }
    }
}
