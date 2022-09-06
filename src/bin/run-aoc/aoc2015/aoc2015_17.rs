use crate::Runner;

pub struct Aoc2015_17 {
    cap: Vec<i32>,
}

impl Aoc2015_17 {
    pub fn new() -> Self {
        Self { cap: Vec::new() }
    }
}

impl Runner for Aoc2015_17 {
    fn name(&self) -> (usize, usize) {
        (2015, 17)
    }

    fn parse(&mut self) {
        self.cap = aoclib::read_lines("input/2015-17.txt")
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(combinations(&self.cap))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

fn sum_eggnog(vec: &[i32], idx: &[usize]) -> i32 {
    idx.iter().fold(0, |a, b| a + vec[*b])
}

// starting with [A, B, C, D]
//  A   A, B    A, B, C     A, B, C, D
//  B   A, C    A, B, D
//  C   A, D    A, C, D
//  D   B, C    B, C, D
//      B, D
//      C, D
fn combinations(vec: &[i32]) -> usize {
    let mut count = 0;
    for combo in 1..=vec.len() {
        let mut idx = Vec::with_capacity(combo);
        for i in 0..combo {
            idx.push(i);
        }

        if sum_eggnog(vec, &idx) == 150 {
            count += 1;
        }

        loop {
            let mut done = true;
            'inc: for inc_index in (0..combo).rev() {
                idx[inc_index] += 1;
                if idx[inc_index] == vec.len() {
                    continue;
                }

                for next in inc_index + 1..combo {
                    idx[next] = idx[next - 1] + 1;
                    if idx[next] == vec.len() {
                        continue 'inc;
                    }
                }
                done = false;
                break; // for inc_index...
            }

            if !done {
                if sum_eggnog(vec, &idx) == 150 {
                    count += 1;
                }
            } else {
                break;
            }
        }
    }

    count
}
