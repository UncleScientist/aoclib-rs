use crate::Runner;
use std::collections::HashMap;

#[derive(Debug)]
struct Bot {
    hold: Vec<Chip>,
    action: (Recipient, Recipient),
}

impl Bot {
    fn new() -> Self {
        Self {
            hold: Vec::new(),
            action: (Recipient::Empty, Recipient::Empty),
        }
    }
}

pub struct Aoc2016_10 {
    bot: HashMap<usize, Bot>,
    bins: HashMap<usize, Vec<i32>>,
}

type Chip = i32;

#[derive(Debug, Copy, Clone)]
enum Recipient {
    Empty,
    Bot(usize),
    Output(usize),
}

impl Recipient {
    fn parse(which: &str, num: &str) -> Self {
        match which {
            "bot" => Self::Bot(num.parse().unwrap()),
            "output" => Self::Output(num.parse().unwrap()),
            _ => panic!("invalid input"),
        }
    }
}

impl Aoc2016_10 {
    pub fn new() -> Self {
        Self {
            bot: HashMap::new(),
            bins: HashMap::new(),
        }
    }
}

impl Runner for Aoc2016_10 {
    fn name(&self) -> (usize, usize) {
        (2016, 10)
    }

    fn parse(&mut self) {
        for line in aoclib::read_lines("input/2016-10.txt") {
            let command = line.split(' ').collect::<Vec<&str>>();
            if command[2] == "goes" {
                let value = command[1].parse::<Chip>().unwrap();
                let bot_num = command[5].parse::<usize>().unwrap();
                let bot = self.bot.entry(bot_num).or_insert_with(Bot::new);
                bot.hold.push(value);
            } else if command[2] == "gives" {
                let bot_num = command[1].parse::<usize>().unwrap();
                let low_dest = Recipient::parse(command[5], command[6]);
                let high_dest = Recipient::parse(command[10], command[11]);
                let bot = self.bot.entry(bot_num).or_insert_with(Bot::new);
                bot.action = (low_dest, high_dest);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut comparison_bot = 0;
        loop {
            let mut bot_num = self
                .bot
                .iter()
                .filter(|(_, bot)| bot.hold.len() == 2)
                .map(|(num, _)| *num)
                .collect::<Vec<usize>>();

            if let Some(num) = bot_num.pop() {
                let bot = self.bot.get_mut(&num).unwrap();

                let (low, high) = if bot.hold[0] < bot.hold[1] {
                    (bot.hold[0], bot.hold[1])
                } else {
                    (bot.hold[1], bot.hold[0])
                };
                let (low_action, high_action) = (bot.action.0, bot.action.1);
                bot.hold.clear();

                if low == 17 && high == 61 {
                    comparison_bot = num;
                }

                match low_action {
                    Recipient::Bot(b) => self.bot.get_mut(&b).unwrap().hold.push(low),
                    Recipient::Output(o) => self.bins.entry(o).or_default().push(low),
                    Recipient::Empty => panic!("oops"),
                }
                match high_action {
                    Recipient::Bot(b) => self.bot.get_mut(&b).unwrap().hold.push(high),
                    Recipient::Output(o) => self.bins.entry(o).or_default().push(high),
                    Recipient::Empty => panic!("oops"),
                }
            } else {
                break;
            }
        }

        crate::output(comparison_bot)
    }

    fn part2(&mut self) -> Vec<String> {
        let bin0 = self.bins.get(&0).unwrap().first().unwrap();
        let bin1 = self.bins.get(&1).unwrap().first().unwrap();
        let bin2 = self.bins.get(&2).unwrap().first().unwrap();
        crate::output(bin0 * bin1 * bin2)
    }
}
