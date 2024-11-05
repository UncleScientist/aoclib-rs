#[derive(Debug, Default)]
pub struct Intcode {
    reset: Vec<i64>,
    memory: Vec<i64>,
    ip: usize,
    running: bool,
}

impl Intcode {
    pub fn new(s: &str) -> Self {
        let reset: Vec<i64> = s
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        Intcode {
            memory: reset.clone(),
            reset,
            ..Self::default()
        }
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            self.step();
            self.ip += 4;
        }
    }

    fn step(&mut self) {
        let inst = self.memory[self.ip];
        match inst {
            1 => {
                let a = self.memory[self.ip + 1] as usize;
                let b = self.memory[self.ip + 2] as usize;
                let dest = self.memory[self.ip + 3] as usize;
                self.memory[dest] = self.memory[a] + self.memory[b];
            }
            2 => {
                let a = self.memory[self.ip + 1] as usize;
                let b = self.memory[self.ip + 2] as usize;
                let dest = self.memory[self.ip + 3] as usize;
                self.memory[dest] = self.memory[a] * self.memory[b];
            }
            99 => {
                self.running = false;
            }
            _ => panic!("instruction {inst} at ip {} unimplemented", self.ip),
        }
    }

    pub(crate) fn poke(&mut self, address: usize, value: i64) {
        self.memory[address] = value;
    }

    pub(crate) fn peek(&self, address: i64) -> i64 {
        self.memory[address as usize]
    }

    pub(crate) fn reset(&mut self) {
        self.memory = self.reset.clone();
        self.ip = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halt() {
        let mut computer = Intcode::new("99");
        computer.run();
        assert!(!computer.running);
    }

    #[test]
    fn test_add() {
        let mut computer = Intcode::new("1,0,2,0,99");
        computer.run();
        assert_eq!(vec![3, 0, 2, 0, 99], computer.memory);
    }

    #[test]
    fn test_multiplication() {
        let mut computer = Intcode::new("2,4,2,0,99");
        computer.run();
        assert_eq!(vec![2 * 99, 4, 2, 0, 99], computer.memory);
    }

    #[test]
    fn test_peek() {
        let computer = Intcode::new("2,4,2,0,99");
        assert_eq!(2, computer.peek(0));
        assert_eq!(4, computer.peek(1));
        assert_eq!(2, computer.peek(2));
        assert_eq!(0, computer.peek(3));
        assert_eq!(99, computer.peek(4));
    }

    #[test]
    fn test_poke() {
        let mut computer = Intcode::new("2,4,2,0,99");
        computer.poke(3, 1234);
        assert_eq!(2, computer.peek(0));
        assert_eq!(4, computer.peek(1));
        assert_eq!(2, computer.peek(2));
        assert_eq!(1234, computer.peek(3));
        assert_eq!(99, computer.peek(4));
    }

    #[test]
    fn test_day_2_part_1() {
        let matrix = vec![
            (
                "1,9,10,3,2,3,11,0,99,30,40,50",
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            ),
            ("1,0,0,0,99", vec![2, 0, 0, 0, 99]),
            ("2,3,0,3,99", vec![2, 3, 0, 6, 99]),
            ("2,4,4,5,99,0", vec![2, 4, 4, 5, 99, 9801]),
            ("1,1,1,4,99,5,6,0,99", vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
        ];
        for entry in matrix {
            let mut computer = Intcode::new(entry.0);
            computer.run();
            assert_eq!(entry.1, computer.memory);
        }
    }

    #[test]
    fn can_reset_program() {
        let mut computer = Intcode::new("1,9,10,3,2,3,11,0,99,30,40,50");
        computer.run();
        computer.reset();
        assert_eq!(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            computer.memory,
        );
        assert_eq!(0, computer.ip);
    }
}
