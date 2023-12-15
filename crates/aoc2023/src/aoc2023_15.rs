use std::{fmt::Display, str::FromStr};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_15 {
    steps: Vec<String>,
}

impl Aoc2023_15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_15 {
    fn name(&self) -> (usize, usize) {
        (2023, 15)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-15.txt");
        self.steps = lines[0].split(',').map(|s| s.to_string()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.steps.iter().fold(0, |acc, step| acc + step.aoc_hash()))
    }

    fn part2(&mut self) -> Vec<String> {
        // let mut boxes = vec![Vec::<Vec<Lens>>::new(); 256];
        let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());

        for step in &self.steps {
            let command: Step = step.parse().unwrap();
            match command {
                Step::Replace(label, focal_length) => {
                    let index = label.aoc_hash();
                    if let Some(lens) = boxes[index].iter_mut().find(|lens| lens.label == label) {
                        lens.focal_length = focal_length;
                    } else {
                        boxes[index].push(Lens::new(label, focal_length));
                    }
                }
                Step::Remove(label) => {
                    let index = label.aoc_hash();
                    boxes[index].retain(|lens| lens.label != label);
                }
            }
        }

        aoclib::output(
            boxes
                .iter()
                .enumerate()
                .map(|(box_number, lens_list)| {
                    lens_list
                        .iter()
                        .enumerate()
                        .map(|(slot, lens)| (box_number + 1) * (slot + 1) * lens.focal_length)
                        .sum::<usize>()
                })
                .sum::<usize>(),
        )
    }
}

trait AocHash {
    fn aoc_hash(&self) -> usize;
}

impl<T: AsRef<str>> AocHash for T {
    fn aoc_hash(&self) -> usize {
        self.as_ref()
            .chars()
            .fold(0, |acc, ch| ((ch as usize + acc) * 17) % 256)
    }
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    fn new<D: Display>(label: D, focal_length: usize) -> Self {
        Self {
            label: label.to_string(),
            focal_length,
        }
    }
}

enum Step {
    Replace(String, usize),
    Remove(String),
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((label, focal_length)) = s.split_once('=') {
            Ok(Self::Replace(
                label.to_string(),
                focal_length.parse().unwrap(),
            ))
        } else if let Some(label) = s.strip_suffix('-') {
            Ok(Self::Remove(label.to_string()))
        } else {
            Err(format!("Cannot parse '{s}'"))
        }
    }
}
