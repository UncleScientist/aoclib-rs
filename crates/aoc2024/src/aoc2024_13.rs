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
    // m = number of times a is pushed
    // n =   ^    ^    ^   b ^    ^
    //
    // px = ax * m + bx * n
    // py = ay * m + by * n
    //
    // m = ( px - bx * n ) / ax
    // m = ( py - by * n ) / ay
    //
    // (px - bx * n)   (py - by * n)
    // ------------- = -------------
    //      ax             ay
    //
    //  ay * (px - bx * n) = ax * (py - by * n)
    //  ay * px - ay * bx * n = ax * py - ax * by * n
    //
    //  ax * by * n - ay * bx * n = ax * py - ay * px
    //
    //  n * (ax * by - ay * bx) = ax * py - ay * px
    //
    //       ax * py - ay * px
    //  n =  -----------------
    //       ax * by - ay * bx
    //
    //  m = (see above)
    fn calc_tokens(&self) -> Option<i64> {
        let a = self.button_a;
        let b = self.button_b;
        let prize = self.prize;

        let n = (a.0 * prize.1 - a.1 * prize.0) / (a.0 * b.1 - a.1 * b.0);
        let m = (prize.0 - b.0 * n) / a.0;

        if (a.0 * m + b.0 * n, a.1 * m + b.1 * n) == prize {
            Some(3 * m + n)
        } else {
            None
        }
    }

    fn _calc_recursive(
        tok_a: i64,
        tok_b: i64,
        a: (i64, i64),
        b: (i64, i64),
        prize: (i64, i64),
    ) -> Option<i64> {
        println!("{a:?} {b:?} {prize:?}");
        if tok_a > 100 || tok_b > 100 {
            return None;
        }
        if prize == (0, 0) {
            return Some(3 * tok_a + tok_b);
        }
        let solution_a = if prize.0 >= a.0 && prize.1 >= a.1 {
            Self::_calc_recursive(tok_a + 1, tok_b, a, b, (prize.0 - a.0, prize.1 - a.1))
        } else {
            None
        };
        if solution_a.is_none() {
            if prize.0 >= b.0 && prize.1 >= b.1 {
                Self::_calc_recursive(tok_a, tok_b + 1, a, b, (prize.0 - b.0, prize.1 - b.1))
            } else {
                None
            }
        } else {
            solution_a
        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example1() {
        let claw = Claw {
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400),
        };
        assert_eq!(Some(280), claw.calc_tokens());
    }
}
