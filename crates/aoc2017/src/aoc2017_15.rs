use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_15 {
    gen_a_start: i64,
    gen_b_start: i64,
}

impl Aoc2017_15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_15 {
    fn name(&self) -> (usize, usize) {
        (2017, 15)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-15.txt");

        self.gen_a_start = lines[0].split_whitespace().nth(4).unwrap().parse().unwrap();
        self.gen_b_start = lines[1].split_whitespace().nth(4).unwrap().parse().unwrap();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut gen_a = Generator::new(GeneratorType::A, self.gen_a_start);
        let mut gen_b = Generator::new(GeneratorType::B, self.gen_b_start);

        aoclib::output(
            (0..40_000_000)
                .map(|_| (gen_a.next().unwrap(), gen_b.next().unwrap()))
                .filter(|(a, b)| (*a & 0xffff) == (*b & 0xffff))
                .count(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut gen_a = Generator::new(GeneratorType::SlowA, self.gen_a_start);
        let mut gen_b = Generator::new(GeneratorType::SlowB, self.gen_b_start);

        aoclib::output(
            (0..5_000_000)
                .map(|_| (gen_a.next().unwrap(), gen_b.next().unwrap()))
                .filter(|(a, b)| (*a & 0xffff) == (*b & 0xffff))
                .count(),
        )
    }
}

#[derive(PartialEq)]
enum GeneratorType {
    A,
    B,
    SlowA,
    SlowB,
}

struct Generator {
    val: i64,
    mul: i64,
    mod_check: i64,
}

impl Generator {
    fn new(gen_type: GeneratorType, val: i64) -> Self {
        let (mul, mod_check) = match gen_type {
            GeneratorType::A => (16807, 0),
            GeneratorType::B => (48271, 0),
            GeneratorType::SlowA => (16807, 4),
            GeneratorType::SlowB => (48271, 8),
        };
        Self {
            val,
            mul,
            mod_check,
        }
    }
}

impl Iterator for Generator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mod_check == 0 {
            self.val = (self.val * self.mul) % 2147483647;
        } else {
            loop {
                self.val = (self.val * self.mul) % 2147483647;
                if self.val % self.mod_check == 0 {
                    break;
                }
            }
        }

        Some(self.val)
    }
}
