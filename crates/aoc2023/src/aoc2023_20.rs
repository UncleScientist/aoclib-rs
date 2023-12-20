use std::collections::{HashMap, VecDeque};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_20 {
    modules: HashMap<String, Module>,
}

impl Aoc2023_20 {
    pub fn new() -> Self {
        Self::default()
    }

    fn pulse(&mut self) -> (i64, i64) {
        let mut queue =
            VecDeque::from_iter([("button".to_string(), "broadcaster".to_string(), false)]);
        let mut pcount = [0, 0];

        while let Some((sender, dest, pulse)) = queue.pop_front() {
            pcount[pulse as usize] += 1;
            if let Some(Module { modtype, outputs }) = self.modules.get_mut(&dest) {
                match modtype {
                    ModuleType::FlipFlop(s) => {
                        if !pulse {
                            *s = !*s;
                            for o in outputs {
                                queue.push_back((dest.clone(), o.clone(), *s));
                            }
                        }
                    }
                    ModuleType::Conjunction(ss) => {
                        ss.insert(sender.clone(), pulse);
                        let out = !ss.values().all(|val| *val);
                        for o in outputs {
                            queue.push_back((dest.clone(), o.clone(), out));
                        }
                    }
                    ModuleType::Broadcaster => {
                        for o in outputs {
                            queue.push_back((dest.clone(), o.clone(), false));
                        }
                    }
                }
            }
        }

        (pcount[0], pcount[1])
    }
}

impl Runner for Aoc2023_20 {
    fn name(&self) -> (usize, usize) {
        (2023, 20)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-20.txt");
        // let lines = aoclib::read_lines("test-input-2");
        let mut conjunctions = HashMap::<String, HashMap<String, bool>>::new();

        for line in lines {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            let rhs = rhs.split(", ").map(|s| s.to_string()).collect();

            if let Some(name) = lhs.strip_prefix('%') {
                self.modules.insert(
                    name.to_string(),
                    Module {
                        modtype: ModuleType::FlipFlop(false),
                        outputs: rhs,
                    },
                );
            } else if let Some(name) = lhs.strip_prefix('&') {
                conjunctions.insert(name.to_string(), HashMap::new());
                self.modules.insert(
                    name.to_string(),
                    Module {
                        modtype: ModuleType::Conjunction(HashMap::new()),
                        outputs: rhs,
                    },
                );
            } else {
                self.modules.insert(
                    lhs.to_string(),
                    Module {
                        modtype: ModuleType::Broadcaster,
                        outputs: rhs,
                    },
                );
            }
        }

        for (name, module) in &self.modules {
            for out in &module.outputs {
                if let Some(conj) = conjunctions.get_mut(out) {
                    conj.insert(name.clone(), false);
                }
            }
        }

        for (name, module) in self.modules.iter_mut() {
            if let Some(conj) = conjunctions.get(name) {
                module.modtype = ModuleType::Conjunction(conj.clone());
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut counts = [0, 0];
        for _ in 0..1000 {
            let (a, b) = self.pulse();
            counts[0] += a;
            counts[1] += b;
        }
        aoclib::output(counts[0] * counts[1])
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct Module {
    modtype: ModuleType,
    outputs: Vec<String>,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}
