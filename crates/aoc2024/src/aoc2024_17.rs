use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_17 {
    device: Computer,
}

impl Aoc2024_17 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_17 {
    fn name(&self) -> (usize, usize) {
        (2024, 17)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-17.txt");
        self.device = Computer::from_lines(&lines);
    }

    fn part1(&mut self) -> Vec<String> {
        let output = self
            .device
            .run()
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>();

        aoclib::output(output.join(","))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug, Default)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    prog: Vec<u8>,
    ip: usize,
}

impl Computer {
    fn from_lines(lines: &[String]) -> Self {
        let a = lines[0].split_once(": ").unwrap().1.parse().unwrap();
        let b = lines[1].split_once(": ").unwrap().1.parse().unwrap();
        let c = lines[2].split_once(": ").unwrap().1.parse().unwrap();
        let prog = lines[3]
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();
        Self {
            a,
            b,
            c,
            prog,
            ip: 0,
        }
    }

    fn step(&mut self) -> Option<i64> {
        let inst = self.prog[self.ip];
        let literal = self.prog[self.ip + 1] as i64;
        let combo = match literal {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => literal,
        };

        self.ip += 2;

        let mut output = None;
        match inst {
            0 => self.a = self.a / (1 << combo),
            1 => self.b = self.b ^ literal,
            2 => self.b = combo % 8,
            3 if self.a != 0 => self.ip = literal as usize,
            3 => {}
            4 => self.b = self.b ^ self.c,
            5 => output = Some(combo % 8),
            6 => self.b = self.a / (1 << combo),
            7 => self.c = self.a / (1 << combo),
            _ => panic!("Invalid instruction {inst}"),
        }

        output
    }

    fn run(&mut self) -> Vec<i64> {
        let mut result = Vec::new();

        while self.ip < self.prog.len() {
            if let Some(output) = self.step() {
                result.push(output);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_computer() {
        let computer = Computer::from_lines(&[
            "Register A: 729".to_string(),
            "Register B: 0".to_string(),
            "Register C: 0".to_string(),
            "Program: 0,1,5,4,3,0".to_string(),
        ]);
        assert_eq!(729, computer.a);
        assert_eq!(0, computer.b);
        assert_eq!(0, computer.c);
        assert_eq!(vec![0, 1, 5, 4, 3, 0], computer.prog);
        assert_eq!(0, computer.ip);
    }

    #[test]
    fn test_output() {
        let mut computer = Computer::from_lines(&[
            "Register A: 729".to_string(),
            "Register B: 0".to_string(),
            "Register C: 0".to_string(),
            "Program: 0,1,5,4,3,0".to_string(),
        ]);

        let result = computer.run();
        assert_eq!(vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0], result);
    }
}
