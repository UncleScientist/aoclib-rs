use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_09 {
    disk: Vec<char>,
}

impl Aoc2024_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_09 {
    fn name(&self) -> (usize, usize) {
        (2024, 9)
    }

    fn parse(&mut self) {
        self.disk = aoclib::read_single_line("input/2024-09.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut fileno = 0_i64;
        let mut disk = Vec::new();
        let mut iter = self.disk.iter();
        while let Some(len) = iter.next() {
            let len = (*len as u8) - b'0';
            disk.extend(vec![fileno; len as usize]);
            fileno += 1;
            if let Some(gap) = iter.next() {
                let gap = (*gap as u8) - b'0';
                disk.extend(vec![-1; gap as usize]);
            }
        }

        while let Some(free_space) = disk.iter().position(|val| *val == -1) {
            while let Some(val) = disk.pop() {
                if val == -1 {
                    continue;
                }
                if free_space >= disk.len() {
                    disk.push(val);
                    break;
                }
                disk[free_space] = val;
                break;
            }
        }

        aoclib::output(
            disk.iter()
                .enumerate()
                .map(|(idx, val)| idx as i64 * val)
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
