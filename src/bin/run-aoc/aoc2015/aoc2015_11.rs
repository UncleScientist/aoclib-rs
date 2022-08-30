use crate::Runner;

pub struct Aoc2015_11 {
    part_1: String,
}

impl Aoc2015_11 {
    pub fn new() -> Self {
        Self {
            part_1: String::from(""),
        }
    }
}

impl Runner for Aoc2015_11 {
    fn parse(&mut self) {}

    fn name(&self) -> (usize, usize) {
        (2015, 11)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut passwd = String::from("hepxcrrq");
        while !valid(&passwd) {
            passwd = incr(&passwd);
        }

        self.part_1 = String::from(&passwd);

        crate::output(passwd)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut passwd = incr(&self.part_1);
        while !valid(&passwd) {
            passwd = incr(&passwd);
        }
        crate::output(passwd)
    }
}

fn valid(input: &str) -> bool {
    let mut has_straight = false;
    let mut pair_count = 0;

    let mut iter = input.chars();
    let mut prev = iter.next().unwrap();
    let mut pair = prev;

    if matches!(prev, 'i' | 'o' | 'l') {
        return false;
    }

    let mut straight_count = 1;
    for c in iter {
        if matches!(c, 'i' | 'o' | 'l') {
            return false;
        }

        if prev as u8 + 1 == c as u8 {
            straight_count += 1;
        } else {
            if straight_count >= 3 {
                has_straight = true;
            }
            straight_count = 1;
        }

        if pair == c {
            pair_count += 1;
            pair = '\0';
        } else {
            pair = c;
        }

        prev = c;
    }

    has_straight && pair_count >= 2
}

fn incr(input: &str) -> String {
    let mut result = String::from("");
    let mut carry = true;

    for c in input.chars().rev() {
        if carry {
            if c == 'z' {
                result.push('a');
            } else {
                result.push(((c as u8) + 1) as char);
                carry = false;
            }
        } else {
            result.push(c);
        }
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hijklmmn_is_invalid() {
        assert!(!valid("hijklmmn"));
    }

    #[test]
    fn abbceffg_is_invalid() {
        assert!(!valid("abbceffg"));
    }

    #[test]
    fn abbcegjk_is_invalid() {
        assert!(!valid("abbcegjk"));
    }

    #[test]
    fn abcdffaa_is_valid() {
        assert!(valid("abcdffaa"));
    }

    #[test]
    fn ghjaabcc_is_valid() {
        assert!(valid("ghjaabcc"));
    }

    #[test]
    fn can_increment_without_carry() {
        assert_eq!(incr("abcde"), "abcdf");
    }

    #[test]
    fn can_increment_with_carry() {
        assert_eq!(incr("abcdz"), "abcea");
    }
}
