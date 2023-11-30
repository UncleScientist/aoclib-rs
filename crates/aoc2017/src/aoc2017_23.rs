use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_23 {
    prog: Vec<Instruction>,
}

impl Aoc2017_23 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_23 {
    fn name(&self) -> (usize, usize) {
        (2017, 23)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2017-23.txt");

        for line in lines {
            self.prog.push(Instruction::parse(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut m = Machine::new(&self.prog);
        while m.step() {}
        aoclib::output(m.mul_count)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut m = Machine::new(&self.prog);
        m.reg[0] = 1;
        while !m.f_cleared && m.step() {}

        /*
         * Re-implement the VM code in rust to avoid the "ineffcient"
         * solution that makes up the puzzle input.
         */
        let mut total = 0;
        'next_num: for n in (m.reg[1]..=m.reg[2]).step_by(17) {
            if n % 2 == 0 {
                total += 1;
            } else {
                let sqrt = (n as f64).sqrt() as i64;
                for d in (3..sqrt).step_by(2) {
                    if n % d == 0 {
                        total += 1;
                        continue 'next_num;
                    }
                }
            }
        }
        aoclib::output(total)
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Set(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Jnz(Value, Value),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let words = s.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "set" => Self::Set(Value::parse_reg(words[1]), Value::parse(words[2])),
            "mul" => Self::Mul(Value::parse_reg(words[1]), Value::parse(words[2])),
            "sub" => Self::Sub(Value::parse_reg(words[1]), Value::parse(words[2])),
            "jnz" => Self::Jnz(Value::parse(words[1]), Value::parse(words[2])),
            _ => panic!("unknown instruction {}", words[0]),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Value {
    Reg(usize),
    Num(i64),
}

impl Value {
    fn parse_reg(s: &str) -> usize {
        let reg_name = s.chars().next().unwrap();
        (reg_name as u8 - b'a') as usize
    }

    fn parse(s: &str) -> Self {
        if let Ok(num) = s.parse::<i64>() {
            Self::Num(num)
        } else {
            Self::Reg(Self::parse_reg(s))
        }
    }
}

#[derive(Debug)]
struct Machine<'a> {
    ip: usize,
    reg: [i64; 8],
    mul_count: usize,
    f_cleared: bool,
    prog: &'a Vec<Instruction>,
}

impl<'a> Machine<'a> {
    fn new(prog: &'a Vec<Instruction>) -> Self {
        Self {
            ip: 0,
            reg: [0; 8],
            mul_count: 0,
            f_cleared: false,
            prog,
        }
    }

    fn get(&self, val: &Value) -> i64 {
        match val {
            Value::Num(n) => *n,
            Value::Reg(r) => self.reg[*r],
        }
    }

    fn step(&mut self) -> bool {
        if self.ip >= self.prog.len() {
            return false;
        }

        let inst = self.prog[self.ip];
        self.ip += 1;

        match inst {
            Instruction::Set(r, val) => {
                let val = self.get(&val);
                self.reg[r] = val;
                if r == 5 && val == 0 {
                    self.f_cleared = true;
                }
            }

            Instruction::Mul(r, val) => {
                self.mul_count += 1;
                let val = self.get(&val);
                self.reg[r] *= val;
            }

            Instruction::Sub(r, val) => {
                let val = self.get(&val);
                self.reg[r] -= val;
            }

            Instruction::Jnz(x, y) => {
                let x = self.get(&x);
                let y = self.get(&y);

                if x != 0 {
                    self.ip -= 1;

                    if y < 0 {
                        let offset = -y as usize;
                        if offset > self.ip {
                            self.ip = self.prog.len();
                            return false;
                        }
                        self.ip -= offset;
                    } else {
                        let offset = y as usize;
                        self.ip += offset;
                        return self.ip < self.prog.len();
                    }
                }
            }
        }

        true
    }
}
