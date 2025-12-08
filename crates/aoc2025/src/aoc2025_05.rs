use aoclib::Runner;

#[derive(Default, Debug)]
pub struct Aoc2025_05 {
    ranges: Vec<Range>,
    ingredients: Vec<usize>,
}

impl Aoc2025_05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_05 {
    fn name(&self) -> (usize, usize) {
        (2025, 5)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2025-05.txt");
        for line in lines {
            if line.contains('-') {
                let (left, right) = line.split_once('-').unwrap();
                self.ranges.push(Range {
                    start: left.parse().unwrap(),
                    end: right.parse().unwrap(),
                });
            } else {
                self.ingredients.push(line.parse().unwrap());
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            self.ingredients
                .iter()
                .filter(|ingredient| self.ranges.iter().any(|range| range.contains(**ingredient)))
                .count(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, ingredient: usize) -> bool {
        self.start <= ingredient && self.end >= ingredient
    }
}
