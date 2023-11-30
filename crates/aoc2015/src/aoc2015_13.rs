use std::collections::{HashMap, HashSet};

use aoclib::Permutations;

use aoclib::Runner;

pub struct Aoc2015_13 {
    data: HashMap<(String, String), i64>,
    people: HashSet<String>,
}

impl Aoc2015_13 {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            people: HashSet::new(),
        }
    }
}

impl Runner for Aoc2015_13 {
    fn name(&self) -> (usize, usize) {
        (2015, 13)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2015-13.txt") {
            let ary = line.split(' ').collect::<Vec<&str>>();
            let first = ary[0].to_string();
            let sign = if ary[2] == "gain" { 1 } else { -1 };
            let hap = ary[3].parse::<i64>().unwrap();
            let mut second = ary[10].to_string();

            // remove full stop at end
            second.pop();

            self.people.insert(first.clone());
            self.people.insert(second.clone());
            self.data.insert((first, second), sign * hap);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.find_happiest())
    }

    fn part2(&mut self) -> Vec<String> {
        self.people.insert("You".to_string());
        aoclib::output(self.find_happiest())
    }
}

impl Aoc2015_13 {
    fn find_happiest(&self) -> i64 {
        let mut all = self
            .people
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let last_person = all.pop().unwrap();

        let mut max_happiness = 0;
        for entry in all.permutations() {
            let mut entry = entry.clone();
            entry.push(last_person.clone());
            max_happiness = self.happiness(entry).max(max_happiness);
        }
        max_happiness
    }

    fn happiness(&self, v: Vec<String>) -> i64 {
        let mut result = 0;
        let wrap = v.len() - 1;

        let first = (v[0].clone(), v[1].clone());
        let last = (v[0].clone(), v[wrap].clone());

        // first
        result += self.data.get(&first).unwrap_or(&0) + self.data.get(&last).unwrap_or(&0);

        // middle
        for entry in 1..wrap {
            let one = (v[entry].clone(), v[entry - 1].clone());
            let two = (v[entry].clone(), v[entry + 1].clone());

            result += self.data.get(&one).unwrap_or(&0) + self.data.get(&two).unwrap_or(&0);
        }

        let first = (v[wrap].clone(), v[wrap - 1].clone());
        let last = (v[wrap].clone(), v[0].clone());

        // last
        result += self.data.get(&first).unwrap_or(&0) + self.data.get(&last).unwrap_or(&0);

        result
    }
}

const _TEST_DATA: &str = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;
