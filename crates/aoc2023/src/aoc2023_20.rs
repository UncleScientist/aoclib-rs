use std::collections::{HashMap, VecDeque};

use aoclib::{Mathemagic, Runner};

#[derive(Default)]
pub struct Aoc2023_20 {
    modules: HashMap<String, Module>,
}

impl Aoc2023_20 {
    pub fn new() -> Self {
        Self::default()
    }

    fn pulse(&mut self, keys: &[String]) -> (u64, u64) {
        let mut queue =
            VecDeque::from_iter([("button".to_string(), "broadcaster".to_string(), false)]);
        let mut pcount = [0, 0];
        let mut this_cycle = 0;

        while let Some((sender, dest, pulse)) = queue.pop_front() {
            if dest == "rx" && !pulse {
                return (0, 0);
            }

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
                        if !keys.is_empty() && out {
                            if let Some(p) = keys.iter().position(|k| k == &dest) {
                                this_cycle = p + 1;
                            }
                        }
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

        if this_cycle != 0 {
            (0, this_cycle as u64 - 1)
        } else {
            (pcount[0], pcount[1])
        }
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
            let (a, b) = self.pulse(&[]);
            counts[0] += a;
            counts[1] += b;
        }
        aoclib::output(counts[0] * counts[1])
    }

    fn part2(&mut self) -> Vec<String> {
        // let cycle = [3847u64, 3851, 4003, 4027];
        // println!("lcm of {cycle:?} = {}", cycle.lcm().unwrap());

        // reset all flip flops and conjunctions
        for Module { modtype, .. } in self.modules.values_mut() {
            match modtype {
                ModuleType::FlipFlop(s) => {
                    *s = false;
                }
                ModuleType::Conjunction(ss) => {
                    for s in ss.values_mut() {
                        *s = false;
                    }
                }
                ModuleType::Broadcaster => {}
            }
        }

        let mut source = None;
        for (name, Module { outputs, .. }) in self.modules.iter() {
            if outputs.contains(&"rx".to_string()) {
                source = Some(name);
                break;
            }
        }
        let Some(source) = source else {
            panic!("could not find predecessor to rx");
        };

        let mut prepre = Vec::new();
        for (name, Module { outputs, .. }) in self.modules.iter() {
            if outputs.contains(source) {
                prepre.push(name.clone());
            }
        }

        let mut loop_count = 0;
        let mut found = 0;
        let mut cycles = vec![0u64; prepre.len()];
        loop {
            loop_count += 1;
            if let (0, c) = self.pulse(&prepre) {
                cycles[c as usize] = loop_count;
                found += 1;
                if found == prepre.len() {
                    break;
                }
            }
        }

        aoclib::output(cycles.lcm().unwrap())
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
