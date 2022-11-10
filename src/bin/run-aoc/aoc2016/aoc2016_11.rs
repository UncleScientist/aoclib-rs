use crate::Runner;

pub struct Aoc2016_11;

const _test_data: &str =
    "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

#[derive(Debug, Default)]
struct Building {
    floor: Vec<Floor>,
    elevator: usize,
}

impl Building {
    fn new(floors: usize) -> Self {
        Self {
            floor: vec![Floor::default(); floors],
            elevator: 0,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Floor {
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
enum Item {
    Microchip(String),
    Generator(String),
}

impl Aoc2016_11 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2016_11 {
    fn name(&self) -> (usize, usize) {
        (2016, 11)
    }

    fn parse(&mut self) {
        let _lines = aoclib::read_lines("input/2016-11.txt");
        let lines = _test_data
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut building = Building::new(4);

        for (f, l) in lines.iter().enumerate() {
            let words = l.split(' ').collect::<Vec<&str>>();
            if words[4] != "nothing" {
                for &next in words.iter().skip(5) {
                    if next == "a"
                        || next == "and"
                        || next.starts_with("micro")
                        || next.starts_with("gen")
                    {
                        continue;
                    }

                    if let Some((left, _)) = next.split_once('-') {
                        building.floor[f]
                            .items
                            .push(Item::Microchip(left.to_string()));
                    } else {
                        building.floor[f]
                            .items
                            .push(Item::Generator(next.to_string()));
                    }
                }
            }
        }

        println!("{building:?}");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
