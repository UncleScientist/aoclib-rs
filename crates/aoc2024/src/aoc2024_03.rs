use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_03 {
    lines: Vec<String>,
}

impl Aoc2024_03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_03 {
    fn name(&self) -> (usize, usize) {
        (2024, 3)
    }

    fn parse(&mut self) {
        self.lines = aoclib::read_lines("input/2024-03.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.lines.iter().map(process).sum::<usize>())
    }

    fn part2(&mut self) -> Vec<String> {
        let s = self.lines.iter().cloned().collect::<String>();
        aoclib::output(process_conditional(s))
    }
}

fn process<S: AsRef<str>>(s: S) -> usize {
    let mut pos = 0;
    let mut total = 0;
    while let Some(loc) = &s.as_ref()[pos..].find("mul(") {
        pos += loc + 4;
        let Some(value) = multiply(&s.as_ref()[pos..]) else {
            continue;
        };
        total += value;
    }

    total
}

fn process_conditional<S: AsRef<str>>(s: S) -> usize {
    let s = s.as_ref();
    let mut pos = 0;
    let mut total = 0;
    let mut enabled = true;
    while let Some(loc) = get_next(&s[pos..]) {
        pos += loc;
        match &s[pos..pos + 3] {
            "mul" => {
                if enabled {
                    if let Some(product) = multiply(&s[pos + 4..]) {
                        total += product;
                    }
                }
            }
            "do(" => enabled = true,
            "don" => enabled = false,
            _ => panic!("bad pattern: {}", &s[pos..pos + 3]),
        }
        pos += 4;
    }

    total
}

fn multiply(s: &str) -> Option<usize> {
    let (left, rest) = s.split_once(',')?;
    let left = left.parse::<usize>().ok()?;
    let (right, _) = rest.split_once(')')?;
    let right = right.parse::<usize>().ok()?;

    Some(left * right)
}

fn get_next(s: &str) -> Option<usize> {
    let mul_loc = s.find("mul(");
    let do_loc = s.find("do()");
    let dont_loc = s.find("don't()");

    [mul_loc, do_loc, dont_loc]
        .iter()
        .filter_map(|loc| *loc)
        .min()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mul() {
        // mul(x,y) .... mul(12;34)
        assert_eq!(Some(25), multiply("5,5)and_the_rest"));
        assert_eq!(None, multiply("5;5)and_the_rest"));
        assert_eq!(None, multiply("5,5xand_the_rest"));
    }

    #[test]
    fn test_line() {
        assert_eq!(
            161,
            process("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
        );
    }

    #[test]
    fn test_conditional() {
        assert_eq!(
            48,
            process_conditional(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )
        );
    }
}
