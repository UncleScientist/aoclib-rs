use aoclib::Runner;
use aoclib::*;
use md5::compute;

const PUZZLE_INPUT: &str = "hhhxzeay";

pub struct Aoc2016_17 {
    vault: Vault,
}

impl Aoc2016_17 {
    pub fn new() -> Self {
        Self {
            vault: Vault::new(""),
        }
    }
}

impl Runner for Aoc2016_17 {
    fn name(&self) -> (usize, usize) {
        (2016, 17)
    }

    fn parse(&mut self) {
        self.vault = Vault::new(PUZZLE_INPUT);
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.vault.search())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(longest_path(&self.vault))
    }
}

impl Nodes for Aoc2016_17 {
    fn get_value(&self, _row: usize, _col: usize) -> usize {
        todo!()
    }

    fn get_width(&self) -> usize {
        todo!()
    }

    fn get_height(&self) -> usize {
        todo!()
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

    fn moves(&self) -> Vec<(Vault, usize)> {
        let mut result = Vec::new();
        // println!("Considering: {self:?}");

        let digest = format!("{:x}", compute(format!("{}{}", self.passcode, self.path)))
            .chars()
            .take(4)
            .collect::<Vec<char>>();

        if self.loc.0 > 0 && digest[0] >= 'b' {
            result.push((
                Vault {
                    passcode: self.passcode.clone(),
                    path: format!("{}U", self.path),
                    loc: (self.loc.0 - 1, self.loc.1),
                },
                1,
            ))
        }

        if self.loc.0 < 3 && digest[1] >= 'b' {
            result.push((
                Vault {
                    passcode: self.passcode.clone(),
                    path: format!("{}D", self.path),
                    loc: (self.loc.0 + 1, self.loc.1),
                },
                1,
            ))
        }

        if self.loc.1 > 0 && digest[2] >= 'b' {
            result.push((
                Vault {
                    passcode: self.passcode.clone(),
                    path: format!("{}L", self.path),
                    loc: (self.loc.0, self.loc.1 - 1),
                },
                1,
            ))
        }

        if self.loc.1 < 3 && digest[3] >= 'b' {
            result.push((
                Vault {
                    passcode: self.passcode.clone(),
                    path: format!("{}R", self.path),
                    loc: (self.loc.0, self.loc.1 + 1),
                },
                1,
            ))
        }

        result
    }

    fn is_win_state(&self) -> bool {
        self.loc == (3, 3)
    }

    fn search(&self) -> String {
        aoclib::ucs(self, |next| next.moves(), |next| next.is_win_state())
            .unwrap()
            .0
            .path
    }
}

fn longest_path(v: &Vault) -> usize {
    let mut longest = 0;
    let mut stack = vec![v.clone()];

    while let Some(state) = stack.pop() {
        for (m, _) in state.moves() {
            if m.is_win_state() {
                longest = longest.max(m.path.len());
            } else {
                stack.push(m.clone());
            }
        }
    }

    longest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_1() {
        let vault = Vault::new("ihgpwlah");
        assert_eq!(vault.search(), "DDRRRD".to_string());
    }

    #[test]
    fn part1_2() {
        let vault = Vault::new("kglvqrro");
        assert_eq!(vault.search(), "DDUDRLRRUDRD".to_string());
    }

    #[test]
    fn part1_3() {
        let vault = Vault::new("ulqzkmiv");
        assert_eq!(vault.search(), "DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string());
    }

    #[test]
    fn part2_1() {
        let vault = Vault::new("ihgpwlah");
        assert_eq!(370, longest_path(&vault));
    }

    #[test]
    fn part2_2() {
        let vault = Vault::new("kglvqrro");
        assert_eq!(492, longest_path(&vault));
    }

    #[test]
    fn part2_3() {
        let vault = Vault::new("ulqzkmiv");
        assert_eq!(830, longest_path(&vault));
    }
}
