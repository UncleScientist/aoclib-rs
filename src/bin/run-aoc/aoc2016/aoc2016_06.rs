use std::collections::HashMap;

use crate::Runner;

pub struct Aoc2016_06;

impl Aoc2016_06 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2016_06 {
    fn name(&self) -> (usize, usize) {
        (2016, 6)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let _test_data = vec![
            "eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
            "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar",
        ];

        let input = aoclib::read_lines("input/2016-06.txt");

        let mut charlist = Vec::new();
        for _ in 0..input[0].len() {
            charlist.push(HashMap::new());
        }

        for entry in &input {
            for (i, c) in entry.chars().enumerate() {
                *charlist[i].entry(c).or_insert(0) += 1;
            }
        }

        let mut result = "".to_string();
        for cl in &charlist {
            let mut v: Vec<(&char, &i32)> = cl.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1));
            result.push(*v[0].0);
        }
        crate::output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
