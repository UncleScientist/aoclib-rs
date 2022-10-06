use crate::Runner;

struct IPAddr {
    addr: Vec<String>,
    hyper: Vec<String>,
}

pub struct Aoc2016_07 {
    ip_addr: Vec<IPAddr>,
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
        for ip in aoclib::read_lines("input/2016-07.txt") {
            self.ip_addr.push(IPAddr::new(&ip));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.ip_addr.iter().filter(|ip| ip.supports_tls()).count())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

impl IPAddr {
    fn new(ip: &str) -> Self {
        let mut addr = Vec::new();
        let mut hyper = Vec::new();

        let (left, rest) = ip.split_once('[').unwrap();
        addr.push(left.to_string());
        for seg in rest.split('[') {
            let (inside, outside) = seg.split_once(']').unwrap();
            hyper.push(inside.to_string());
            addr.push(outside.to_string());
        }
        Self { addr, hyper }
    }

    fn supports_tls(&self) -> bool {
        for h in &self.hyper {
            if has_abba(h) {
                return false;
            }
        }

        for a in &self.addr {
            if has_abba(a) {
                return true;
            }
        }

        false
    }
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
        assert!(IPAddr::new("abba[mnop]qrst").supports_tls());
    }

    #[test]
    fn does_not_support() {
        assert!(!IPAddr::new("abcd[bddb]xyyx").supports_tls());
    }

    #[test]
    fn must_be_different() {
        assert!(!IPAddr::new("aaaa[qwer]tyui").supports_tls());
    }

    #[test]
    fn middle_works() {
        assert!(IPAddr::new("ioxxoj[asdfgh]zxcvbn").supports_tls());
    }

    #[test]
    fn end_works() {
        assert!(IPAddr::new("asdfasf[qwoeiruqwe]abba").supports_tls());
    }
}
