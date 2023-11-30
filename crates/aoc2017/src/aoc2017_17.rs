use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_17 {
    // insert items here (or not, i'm not the boss of you)
}

impl Aoc2017_17 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_17 {
    fn name(&self) -> (usize, usize) {
        (2017, 17)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let mut sl = Spinlock::new(349);

        for i in 1..=2017 {
            sl.one_step(i);
        }

        let pos = (sl.buffer.iter().position(|&x| x == 2017).unwrap() + 1) % sl.buffer.len();
        aoclib::output(sl.buffer[pos])
    }

    fn part2(&mut self) -> Vec<String> {
        let mut loc = 0;
        let mut result = 0;
        for i in 1..50_000_000 {
            loc = (1 + (loc + 349)) % (i + 1);
            if loc == 0 {
                result = i + 1;
            }
        }

        aoclib::output(result)
    }
}

#[derive(Debug)]
struct Spinlock {
    buffer: Vec<usize>,
    step: usize,
    current_pos: usize,
}

impl Spinlock {
    fn new(step: usize) -> Self {
        Self {
            buffer: vec![0],
            step,
            current_pos: 0,
        }
    }

    fn one_step(&mut self, num: usize) {
        self.current_pos = (1 + (self.current_pos + self.step)) % self.buffer.len();
        self.buffer.insert(self.current_pos, num);
    }

    fn _print(&self) {
        for (i, c) in self.buffer.iter().enumerate() {
            if i == self.current_pos {
                print!("({c:2})");
            } else {
                print!(" {c:2} ");
            }
        }
        println!();
    }
}
