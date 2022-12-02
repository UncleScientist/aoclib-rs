use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_02 {
    moves: Vec<(char, char)>,
}

impl Aoc2022_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_02 {
    fn name(&self) -> (usize, usize) {
        (2022, 2)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2022-02.txt");
        for l in lines {
            let (opp, me) = l.split_once(' ').unwrap();
            self.moves.push((
                opp.chars().take(1).next().unwrap(),
                me.chars().take(1).next().unwrap(),
            ));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut score = 0;

        for m in &self.moves {
            let subscore = match m.1 {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => panic!("bad"),
            };
            let op_move = match m.0 {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => panic!("bad"),
            };
            score += subscore
                + if subscore == op_move {
                    3
                } else if subscore == 1 && op_move == 3
                    || subscore == 2 && op_move == 1
                    || subscore == 3 && op_move == 2
                {
                    6
                } else {
                    0
                };
        }
        crate::output(score)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut score = 0;

        for m in &self.moves {
            let op_move = match m.0 {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => panic!("bad"),
            };
            score += match m.1 {
                'X' => {
                    if op_move == 1 {
                        3
                    } else {
                        op_move - 1
                    }
                }
                'Y' => 3 + op_move,
                'Z' => 6 + if op_move == 3 { 1 } else { op_move + 1 },
                _ => panic!("very bad"),
            };
        }

        crate::output(score)
    }
}
