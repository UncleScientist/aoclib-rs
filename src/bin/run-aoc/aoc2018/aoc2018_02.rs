use crate::Runner;

#[derive(Default)]
pub struct Aoc2018_02 {
    boxid: Vec<String>,
}

impl Aoc2018_02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2018_02 {
    fn name(&self) -> (usize, usize) {
        (2018, 2)
    }

    fn parse(&mut self) {
        self.boxid = aoclib::read_lines("input/2018-02.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut count_of_twos = 0;
        let mut count_of_threes = 0;
        for id in &self.boxid {
            let mut counts = [0usize; 26];
            for c in id.chars() {
                counts[((c as u8) - b'a') as usize] += 1;
            }
            count_of_twos += counts.contains(&2) as i32;
            count_of_threes += counts.contains(&3) as i32;
        }
        crate::output(count_of_twos * count_of_threes)
    }

    fn part2(&mut self) -> Vec<String> {
        for i in 0..self.boxid.len() - 1 {
            'next_pair: for j in i + 1..self.boxid.len() {
                let mut count = 0;
                let mut diff = 'a';
                for (a, b) in self.boxid[i].chars().zip(self.boxid[j].chars()) {
                    if a != b {
                        if count > 0 {
                            continue 'next_pair;
                        }
                        count += 1;
                        diff = a;
                    }
                }

                if count == 1 {
                    let pos = self.boxid[i].find(diff).unwrap();
                    let mut common = self.boxid[i].clone();
                    common.remove(pos);
                    return crate::output(common);
                }
            }
        }
        crate::output("** no answer found **")
    }
}
