use aoclib::Runner;

pub struct Aoc2016_20 {
    blacklist: Vec<Range>,
}

impl Aoc2016_20 {
    pub fn new() -> Self {
        Self {
            blacklist: Vec::new(),
        }
    }
}

impl Runner for Aoc2016_20 {
    fn name(&self) -> (usize, usize) {
        (2016, 20)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2016-20.txt");
        for l in lines {
            let (low, high) = l.split_once('-').unwrap();
            let low = low.parse::<u64>().unwrap();
            let high = high.parse::<u64>().unwrap();
            self.blacklist = insert_interval(&self.blacklist, (low, high));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        if self.blacklist[0].0 > 0 {
            aoclib::output(0)
        } else {
            aoclib::output(self.blacklist[0].1 + 1)
        }
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(count_allowed(&self.blacklist))
    }
}

type Range = (u64, u64);

fn insert_interval(src: &Vec<Range>, insert: Range) -> Vec<Range> {
    let mut result = Vec::new();
    let mut to_insert = insert;
    let mut pushed = false;

    for entry in src {
        if entry.1 + 1 < to_insert.0 {
            result.push(*entry);
        } else if entry.0 > to_insert.1 + 1 {
            if !pushed {
                result.push(to_insert);
                pushed = true;
            }
            result.push(*entry);
        } else {
            to_insert.0 = to_insert.0.min(entry.0);
            to_insert.1 = to_insert.1.max(entry.1);
        }
    }

    if !pushed {
        result.push(to_insert);
    }

    result
}

fn count_allowed(bl: &[Range]) -> usize {
    let mut allowed: usize = 0;
    for entry in bl.windows(2) {
        allowed += entry[1].0 as usize - entry[0].1 as usize - 1;
    }
    allowed + (u32::MAX as u64 - bl[bl.len() - 1].1) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_test() {
        let r: Range = (0, 3);
        assert_eq!(vec![r], insert_interval(&Vec::new(), r));
    }

    #[test]
    fn second_insert() {
        let first = insert_interval(&vec![], (5, 8));
        let second = insert_interval(&first, (0, 2));
        let third = insert_interval(&second, (4, 7));
        assert_eq!(vec![(0, 2), (4, 8)], third);
    }

    #[test]
    fn check_inclusive_endings() {
        assert_eq!(vec![(1, 6)], insert_interval(&vec![(1, 3)], (4, 6)));
    }

    #[test]
    fn check_allowed() {
        assert_eq!(
            (u32::MAX - 8 + 1) as usize,
            count_allowed(&vec![(0, 2), (4, 8)])
        );
    }
}
