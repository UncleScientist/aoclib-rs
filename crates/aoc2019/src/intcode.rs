use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Intcode {
    reset: Vec<i64>,
    memory: Vec<i64>,
    ip: usize,
    running: bool,
    inputq: VecDeque<i64>,
    outputq: VecDeque<i64>,
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
            let offset = self.step();
            self.ip = self.ip.wrapping_add(offset as usize);
        }
    }

    pub fn run_until_output(&mut self) -> Option<i64> {
        self.running = true;

        while self.running {
            let offset = self.step();
            self.ip = self.ip.wrapping_add(offset as usize);
            if let Some(output) = self.outputq.pop_front() {
                return Some(output);
            }
        }

        None
    }

    fn read(&self, address: usize, mode: AddressingMode) -> i64 {
        match mode {
            AddressingMode::Position => {
                let location = self.memory[address] as usize;
                self.memory[location]
            }
            AddressingMode::Immediate => self.memory[address],
        }
    }

    fn step(&mut self) -> i64 {
        let inst = self.memory[self.ip];

        let modea: AddressingMode = ((inst / 100) % 10).into();
        let modeb: AddressingMode = ((inst / 1000) % 10).into();
        let modec: AddressingMode = ((inst / 10000) % 10).into();
        let inst = inst % 100;

        match inst {
            1 => {
                let a = self.read(self.ip + 1, modea);
                let b = self.read(self.ip + 2, modeb);
                assert_eq!(AddressingMode::Position, modec);
                let dest = self.memory[self.ip + 3] as usize;
                self.memory[dest] = a + b;
                4
            }
            2 => {
                let a = self.read(self.ip + 1, modea);
                let b = self.read(self.ip + 2, modeb);
                assert_eq!(AddressingMode::Position, modec);
                let dest = self.memory[self.ip + 3] as usize;
                self.memory[dest] = a * b;
                4
            }
            3 => {
                assert_eq!(AddressingMode::Position, modea);
                let a = self.memory[self.ip + 1];
                let val = self.inputq.pop_front().unwrap();
                self.memory[a as usize] = val;
                2
            }
            4 => {
                let a = self.read(self.ip + 1, modea);
                self.outputq.push_back(a);
                2
            }
            5 => {
                let a = self.read(self.ip + 1, modea);
                let b = self.read(self.ip + 2, modeb);
                if a != 0 {
                    b - self.ip as i64
                } else {
                    3
                }
            }
            6 => {
                let a = self.read(self.ip + 1, modea);
                let b = self.read(self.ip + 2, modeb);
                if a == 0 {
                    b - self.ip as i64
                } else {
                    3
                }
            }
            7 => {
                let a = self.read(self.ip + 1, modea);
                let b = self.read(self.ip + 2, modeb);
                assert_eq!(AddressingMode::Position, modec);
                let dest = self.memory[self.ip + 3] as usize;
                self.memory[dest] = (a < b) as i64;
                4
            }
            8 => {
                let a = self.read(self.ip + 1, modea);
                let b = self.read(self.ip + 2, modeb);
                assert_eq!(AddressingMode::Position, modec);
                let dest = self.memory[self.ip + 3] as usize;
                self.memory[dest] = (a == b) as i64;
                4
            }
            99 => {
                self.running = false;
                0
            }
            _ => panic!(
                "instruction {inst} at ip {} unimplemented: {:?}",
                self.ip, self.memory
            ),
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

    pub(crate) fn push(&mut self, value: i64) {
        self.inputq.push_back(value);
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum AddressingMode {
    Position = 0,
    Immediate = 1,
}

impl From<i64> for AddressingMode {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("invalid addressing mode {value}"),
        }
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

    #[test]
    fn copies_input_to_output() {
        let mut computer = Intcode::new("3,0,4,0,99");

        computer.push(123);
        assert_eq!(Some(123), computer.run_until_output());
        assert_eq!(None, computer.run_until_output());
        assert!(!computer.running);
    }

    #[test]
    fn test_addressing_mode() {
        let mut computer = Intcode::new("1002,4,3,4,33");
        computer.run();
        assert_eq!(99, computer.memory[4]);
    }

    #[test]
    fn test_equals_eight() {
        let mut computer = Intcode::new("3,9,8,9,10,9,4,9,99,-1,8");

        computer.push(8);
        assert_eq!(Some(1), computer.run_until_output());
        computer.reset();
        computer.push(9);
        assert_eq!(Some(0), computer.run_until_output());
    }

    #[test]
    fn test_less_than_eight() {
        let mut computer = Intcode::new("3,9,7,9,10,9,4,9,99,-1,8");

        computer.push(7);
        assert_eq!(Some(1), computer.run_until_output());

        computer.reset();
        computer.push(8);
        assert_eq!(Some(0), computer.run_until_output());

        computer.reset();
        computer.push(9);
        assert_eq!(Some(0), computer.run_until_output());
    }

    #[test]
    fn test_jump_mode() {
        let mut computer = Intcode::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");

        computer.push(0);
        assert_eq!(Some(0), computer.run_until_output());

        computer.reset();
        computer.push(123);
        assert_eq!(Some(1), computer.run_until_output());
    }

    #[test]
    fn test_three_way_for_eight() {
        let mut computer = Intcode::new("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");

        computer.push(3);
        assert_eq!(Some(999), computer.run_until_output());

        computer.reset();
        computer.push(8);
        assert_eq!(Some(1000), computer.run_until_output());

        computer.reset();
        computer.push(123);
        assert_eq!(Some(1001), computer.run_until_output());
    }
}
