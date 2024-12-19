use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_19 {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Aoc2024_19 {
    pub fn new() -> Self {
        Self::default()
    }

    fn can_make_design(&self, design: &str) -> bool {
        if design.is_empty() {
            return true;
        }

        for pattern in &self.patterns {
            if design.starts_with(pattern) && self.can_make_design(&design[pattern.len()..]) {
                return true;
            }
        }

        false
    }
}

impl Runner for Aoc2024_19 {
    fn name(&self) -> (usize, usize) {
        (2024, 19)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-19.txt");
        self.patterns = lines[0]
            .split(", ")
            .map(|pattern| pattern.to_string())
            .collect::<Vec<_>>();
        self.designs = Vec::from(&lines[1..]);
    }

    fn part1(&mut self) -> Vec<String> {
        println!("{}", self.designs.len());
        aoclib::output(
            self.designs
                .iter()
                .filter(|design| self.can_make_design(design))
                .count(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}
