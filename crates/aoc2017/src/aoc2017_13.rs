use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_13 {
    firewall: Firewall,
}

impl Aoc2017_13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_13 {
    fn name(&self) -> (usize, usize) {
        (2017, 13)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-13.txt");

        for line in lines {
            let (layer, depth) = line.split_once(": ").unwrap();
            self.firewall
                .add(layer.parse().unwrap(), depth.parse().unwrap());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.firewall.severity(0).unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(
            (0usize..)
                .find(|&delay| Some(0) == self.firewall.severity(delay))
                .unwrap(),
        )
    }
}

#[derive(Default)]
struct Firewall {
    scanners: Vec<(usize, usize)>,
}

impl Firewall {
    fn add(&mut self, layer: usize, depth: usize) {
        self.scanners.push((layer, depth));
    }

    fn severity(&self, delay: usize) -> Option<usize> {
        let mut severity = 0;
        let mut range = 0;
        let mut picoseconds = delay;

        for &(layer, depth) in &self.scanners {
            let dist = layer - range;

            range += dist;
            picoseconds += dist;

            if (picoseconds % ((depth - 1) * 2)) == 0 {
                if delay != 0 {
                    return None;
                }
                severity += layer * depth;
            }
        }

        Some(severity)
    }
}
