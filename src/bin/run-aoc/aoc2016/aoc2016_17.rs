use crate::Runner;
use aoclib::*;
use md5::compute;

const PUZZLE_INPUT: &str = "hhhxzeay";

pub struct Aoc2016_17;

impl Aoc2016_17 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Aoc2016_17 {
    fn name(&self) -> (usize, usize) {
        (2016, 17)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let vault = Vault::new(PUZZLE_INPUT);
        crate::output(dijkstra_search(&vault).unwrap().0.path)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vault {
    passcode: String,
    path: String,
    loc: (usize, usize),
}

impl Vault {
    fn new(passcode: &str) -> Self {
        Self {
            passcode: passcode.to_string(),
            path: "".to_string(),
            loc: (0, 0), // row, col
        }
    }
}

impl DijkstraSearch for Vault {
    fn moves(&self) -> Vec<Vault> {
        let mut result = Vec::new();
        // println!("Considering: {self:?}");

        let digest = format!("{:x}", compute(format!("{}{}", self.passcode, self.path)))
            .chars()
            .take(4)
            .collect::<Vec<char>>();

        if self.loc.0 > 0 && digest[0] >= 'b' {
            result.push(Vault {
                passcode: self.passcode.clone(),
                path: format!("{}U", self.path),
                loc: (self.loc.0 - 1, self.loc.1),
            })
        }

        if self.loc.0 < 3 && digest[1] >= 'b' {
            result.push(Vault {
                passcode: self.passcode.clone(),
                path: format!("{}D", self.path),
                loc: (self.loc.0 + 1, self.loc.1),
            })
        }

        if self.loc.1 > 0 && digest[2] >= 'b' {
            result.push(Vault {
                passcode: self.passcode.clone(),
                path: format!("{}L", self.path),
                loc: (self.loc.0, self.loc.1 - 1),
            })
        }

        if self.loc.1 < 3 && digest[3] >= 'b' {
            result.push(Vault {
                passcode: self.passcode.clone(),
                path: format!("{}R", self.path),
                loc: (self.loc.0, self.loc.1 + 1),
            })
        }

        result
    }

    fn is_win_state(&self) -> bool {
        self.loc == (3, 3)
    }
}
