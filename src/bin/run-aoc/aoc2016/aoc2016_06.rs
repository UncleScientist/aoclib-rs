use std::collections::HashMap;

use crate::Runner;

pub struct Aoc2016_06 {
    charlist: Vec<HashMap<char, i32>>,
}

impl Aoc2016_06 {
    pub fn new() -> Self {
        Self {
            charlist: Vec::new(),
        }
    }
}

impl Runner for Aoc2016_06 {
    fn name(&self) -> (usize, usize) {
        (2016, 6)
    }

    fn parse(&mut self) {
        let _test_data = vec![
            "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
            "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar",
        ];

        let input = aoclib::read_lines("input/2016-06.txt");

        for _ in 0..input[0].len() {
            self.charlist.push(HashMap::new());
        }

        for entry in &input {
            for (i, c) in entry.chars().enumerate() {
                *self.charlist[i].entry(c).or_insert(0) += 1;
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut result = "".to_string();
        for cl in &self.charlist {
            let mut v: Vec<(&char, &i32)> = cl.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1));
            result.push(*v[0].0);
        }
        crate::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut result = "".to_string();
        for cl in &self.charlist {
            let mut v: Vec<(&char, &i32)> = cl.iter().collect();
            v.sort_by(|a, b| a.1.cmp(b.1));
            result.push(*v[0].0);
        }
        crate::output(result)
    }
}
