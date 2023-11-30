use aoclib::Runner;

pub struct Aoc2015_10 {
    part_1_result: String,
}

impl Aoc2015_10 {
    pub fn new() -> Self {
        Self {
            part_1_result: String::from(""),
        }
    }
}

impl Runner for Aoc2015_10 {
    fn parse(&mut self) {}

    fn name(&self) -> (usize, usize) {
        (2015, 10)
    }

    fn part1(&mut self) -> Vec<String> {
        self.part_1_result = String::from("1321131112");
        for _ in 0..40 {
            self.part_1_result = one_round(&self.part_1_result);
        }
        aoclib::output(self.part_1_result.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut part2 = self.part_1_result.clone();
        for _ in 0..10 {
            part2 = one_round(&part2);
        }
        aoclib::output(part2.len())
    }
}

fn one_round(input: &str) -> String {
    let mut result = String::from("");
    let mut iter = input.chars().peekable();

    while let Some(cur) = iter.next() {
        let mut count = 1;
        while let Some(next) = iter.peek() {
            if *next == cur {
                count += 1;
                iter.next();
            } else {
                break;
            }
        }
        result.push_str(format!("{count}{cur}").as_str());
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rounds() {
        let mut say = String::from("1");
        say = one_round(&say);
        assert_eq!(say, "11".to_string());
        say = one_round(&say);
        assert_eq!(say, "21".to_string());
        say = one_round(&say);
        assert_eq!(say, "1211".to_string());
        say = one_round(&say);
        assert_eq!(say, "111221".to_string());
        say = one_round(&say);
        assert_eq!(say, "312211".to_string());
    }
}
