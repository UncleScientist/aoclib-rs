use std::str::FromStr;

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_13 {
    claws: Vec<Claw>,
}

impl Aoc2024_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_13 {
    fn name(&self) -> (usize, usize) {
        (2024, 13)
    }

    fn parse(&mut self) {
        let data = aoclib::read_text_records("input/2024-13.txt");
        self.claws = data.iter().map(|claw| claw.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.claws.iter().filter_map(Claw::calc_tokens).sum::<i64>())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, Default)]
struct Claw {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Claw {
    fn calc_tokens(&self) -> Option<i64> {
        Some(1)
    }
}

impl FromStr for Claw {
    type Err = ();

    fn from_str(claw_data: &str) -> Result<Self, Self::Err> {
        let lines = claw_data.split('\n').collect::<Vec<_>>();

        fn get_xy(s: &str) -> (i64, i64) {
            let (_, xy) = s.split_once(": ").unwrap();
            let (x, y) = xy.split_once(", ").unwrap();
            (
                x[2..].parse::<i64>().unwrap(),
                y[2..].parse::<i64>().unwrap(),
            )
        }

        let button_a = get_xy(&lines[0]);
        let button_b = get_xy(&lines[1]);
        let prize = get_xy(&lines[2]);

        Ok(Self {
            button_a,
            button_b,
            prize,
        })
    }
}
