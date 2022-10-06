use crate::Runner;

pub struct Aoc2016_07 {
    ip_addr: Vec<String>,
}

impl Aoc2016_07 {
    pub fn new() -> Self {
        Self {
            ip_addr: Vec::new(),
        }
    }
}

impl Runner for Aoc2016_07 {
    fn name(&self) -> (usize, usize) {
        (2016, 7)
    }

    fn parse(&mut self) {
        self.ip_addr = aoclib::read_lines("input/2016-07.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.ip_addr.iter().filter(|ip| supports_tls(ip)).count())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

fn supports_tls(ip: &str) -> bool {
    let mut addr = Vec::new();
    let mut hyper = Vec::new();

    let (left, rest) = ip.split_once('[').unwrap();
    addr.push(left);
    for seg in rest.split('[') {
        let (inside, outside) = seg.split_once(']').unwrap();
        hyper.push(inside);
        addr.push(outside);
    }

    for h in hyper {
        if has_abba(h) {
            return false;
        }
    }

    for a in addr {
        if has_abba(a) {
            return true;
        }
    }

    false
}

fn has_abba(s: &str) -> bool {
    let ch: Vec<char> = s.chars().collect();
    for idx in 0..ch.len() - 3 {
        if ch[idx] == ch[idx + 1] {
            continue;
        }

        if ch[idx] == ch[idx + 3] && ch[idx + 1] == ch[idx + 2] {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_abba_works() {
        assert!(has_abba("abba"));
    }

    #[test]
    fn supports() {
        assert!(supports_tls("abba[mnop]qrst"));
    }

    #[test]
    fn does_not_support() {
        assert!(!supports_tls("abcd[bddb]xyyx"));
    }

    #[test]
    fn must_be_different() {
        assert!(!supports_tls("aaaa[qwer]tyui"));
    }

    #[test]
    fn middle_works() {
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn end_works() {
        assert!(supports_tls("asdfasf[qwoeiruqwe]abba"));
    }
}
