use std::{collections::HashMap, str::FromStr};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_24 {
    system: HashMap<String, Gate>,
    x: Option<u64>,
    y: Option<u64>,
    swapper: HashMap<String, String>,
}

impl Aoc2024_24 {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_number(&self, which: char) -> u64 {
        let mut wires = self
            .system
            .keys()
            .filter(|key| key.starts_with(which))
            .collect::<Vec<_>>();
        wires.sort();

        let mut result = 0;
        for (shift, wire) in wires.iter().enumerate() {
            if let Some(v) = self.get_value(wire, self.system.len()) {
                result |= v << shift;
            } else {
                return u64::MAX;
            }
        }
        result
    }

    fn get_value(&self, wire: &str, depth: usize) -> Option<u64> {
        if depth == 0 {
            return None;
        }

        let wire = if let Some(alt) = self.swapper.get(wire) {
            alt
        } else {
            wire
        };

        /*
        let wire = if wire == "rts" {
            "z07"
        } else if wire == "z07" {
            "rts"
        } else {
            wire
        };
        */

        Some(match &self.system[wire] {
            Gate::And(a, b) => self.get_value(a, depth - 1)? & self.get_value(b, depth - 1)?,
            Gate::Or(a, b) => self.get_value(a, depth - 1)? | self.get_value(b, depth - 1)?,
            Gate::Xor(a, b) => self.get_value(a, depth - 1)? ^ self.get_value(b, depth - 1)?,
            Gate::Const(c) => {
                let bit = &wire[1..].parse::<u64>().unwrap();
                if wire.starts_with('x') {
                    if let Some(x) = self.x {
                        ((x & (1 << bit)) != 0) as u64
                    } else {
                        *c
                    }
                } else if let Some(y) = self.y {
                    ((y & (1 << bit)) != 0) as u64
                } else {
                    *c
                }
            }
        })
    }

    fn find_xor_gate(&self, xwire: &str, ywire: &str) -> Option<(&String, &Gate)> {
        for (k, v) in &self.system {
            if matches!(v, Gate::Xor(left, right)
                        if (*left == xwire && *right == ywire)
                        || (*left == ywire && *right == xwire))
            {
                return Some((k, v));
            }
        }
        None
    }

    fn find_and_gate(&self, xwire: &str, ywire: &str) -> Option<(&String, &Gate)> {
        for (k, v) in &self.system {
            if matches!(v, Gate::And(left, right)
                        if (*left == xwire && *right == ywire)
                        || (*left == ywire && *right == xwire))
            {
                return Some((k, v));
            }
        }
        None
    }

    // find any gate with 'wire' as an input
    fn find_gates(&self, wire: &str) -> Vec<(&String, &Gate)> {
        let mut result = Vec::new();
        for (k, v) in &self.system {
            if matches!(v, Gate::And(left, right) if *left == wire || *right == wire)
                || matches!(v, Gate::Or(left, right) if *left == wire || *right == wire)
                || matches!(v, Gate::Xor(left, right) if *left == wire || *right == wire)
            {
                result.push((k, v));
            }
        }
        result
    }
}

impl Runner for Aoc2024_24 {
    fn name(&self) -> (usize, usize) {
        (2024, 24)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2024-24.txt");
        for line in lines {
            if let Some((left, right)) = line.split_once(": ") {
                // a value
                self.system.insert(left.into(), right.parse().unwrap());
            } else if let Some((left, right)) = line.split_once(" -> ") {
                // a gate
                self.system.insert(right.into(), left.parse().unwrap());
            } else {
                // a bug
                panic!("bug in line {line}");
            }
        }
        self.x = Some(self.get_number('x'));
        self.y = Some(self.get_number('y'));
        println!("{:?}, {:?}", self.x, self.y);
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.get_number('z'))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut possible_pairs = Vec::new();
        let ys = self
            .system
            .keys()
            .filter(|wire| wire.starts_with('y'))
            .count();
        for y in 0..ys {
            self.x = Some(0);
            self.y = Some(1 << y);
            let sum = self.get_number('z');

            if sum != 1 << y {
                println!("-----");
                println!("bit {y}: expected {}, got {sum}", 1u64 << y);
                let (xwire, ywire) = (format!("x{y:02}"), format!("y{y:02}"));
                let mut output_list = Vec::new();

                // The "AND" gate
                let Some((output, _)) = self.find_and_gate(&xwire, &ywire) else {
                    panic!("can't find and gate for {xwire},{ywire}");
                };
                output_list.push(output.clone());
                let gate_list = self.find_gates(output);
                for (output, _) in gate_list {
                    output_list.push(output.clone());
                }

                // The "XOR" gate
                let Some((output, _)) = self.find_xor_gate(&xwire, &ywire) else {
                    panic!("can't find and gate for {xwire},{ywire}");
                };
                let gate_list = self.find_gates(output);
                for (g, _) in gate_list {
                    output_list.push(g.clone());
                }
                println!("swap candidates: {output_list:?}");
                for i in 0..output_list.len() - 1 {
                    for j in i + 1..output_list.len() {
                        self.swapper
                            .insert(output_list[i].clone(), output_list[j].clone());
                        self.swapper
                            .insert(output_list[j].clone(), output_list[i].clone());
                        let sum = self.get_number('z');
                        self.x = Some(0);
                        self.y = Some(1 << y);
                        if sum != 1 << y {
                            self.swapper.remove(&output_list[i]);
                            self.swapper.remove(&output_list[j]);
                            continue;
                        }
                        self.x = Some(1 << y);
                        self.y = Some(0);
                        let sum = self.get_number('z');
                        if sum != 1 << y {
                            self.swapper.remove(&output_list[i]);
                            self.swapper.remove(&output_list[j]);
                            continue;
                        }
                        self.x = Some(1 << y);
                        self.y = Some(1 << y);
                        let sum = self.get_number('z');
                        if sum != 1 << (y + 1) {
                            self.swapper.remove(&output_list[i]);
                            self.swapper.remove(&output_list[j]);
                            continue;
                        }
                        println!("Found pair: {}, {}", output_list[i], output_list[j]);
                        possible_pairs.push((output_list[i].clone(), output_list[j].clone()));
                        self.swapper.remove(&output_list[i]);
                        self.swapper.remove(&output_list[j]);
                    }
                }
            }
        }
        println!("possible swapping pairs: {possible_pairs:?}");

        // hardcoding based on output
        for pair in [
            ("z07", "rts"),
            ("z12", "jpj"),
            ("z26", "kgj"),
            ("chv", "vvw"),
        ] {
            self.swapper.insert(pair.0.into(), pair.1.into());
            self.swapper.insert(pair.1.into(), pair.0.into());
        }
        println!("{:?}", self.swapper);

        self.x = None;
        self.y = None;

        self.x = Some(self.get_number('x'));
        self.y = Some(self.get_number('y'));
        let (Some(x), Some(y)) = (self.x, self.y) else {
            panic!("oops");
        };
        println!("sum {x} + {y} = {}", x + y);
        let z = self.get_number('z');
        println!(" == {z}");
        let mut keys = self.swapper.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        aoclib::output(keys.join(","))
    }
}

// CarryIn A B | SumOut CarryOut
//       0 0 0 |    0      0
//       0 0 1 |    1      0
//       0 1 0 |    1      0
//       0 1 1 |    0      1
//       1 0 0 |    1      0
//       1 0 1 |    0      1
//       1 1 0 |    0      1
//       1 1 1 |    1      1

// x00 AND y00 -> whb           (to carry)
// x00 XOR y00 ->       z00     (sumOut)

// x01 AND y01 -> bdf           (to carry)
// x01 XOR y01 -> jjd
// jjd XOR whb ->       z01     (sumOut)
// jjd AND whb -> wbw           (to carry)
// bdf OR  wbw -> qkf           (carryOut)

// x06 AND y06 -> ssv
// x06 XOR y06 -> qmt
// gtn XOR qmt ->       z06
// gtn AND qmt -> jmk
// jmk OR  ssv -> rkv
//
// x07 AND y07 -> sdj
// x07 XOR y07 -> cds
// cds XOR rkv -> rts  z07
// cds AND rkv -> nph
// sdj OR  nph -> z07  rts

// y12 AND x12 -> mqn
// x12 XOR y12 -> nft
// ksn XOR nft -> jpj
// ksn AND nft -> z12
// mqn OR  jpj -> bwg

// x26 AND y26 -> z26
// x26 XOR y26 -> bvp
// bvp XOR gdb -> kgj
// bvp AND gdb -> stc
// kgj OR  stc -> www

// y34 AND x34 -> chv
// x34 XOR y34 -> vvw
// chv XOR fqf -> z34
// fqf AND chv -> cwh
// cwh OR  vvw -> ttb

#[derive(Debug)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Const(u64),
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<u64>() {
            Ok(Self::Const(value))
        } else {
            let words = s.split_whitespace().collect::<Vec<_>>();
            Ok(match words[1] {
                "AND" => Self::And(words[0].into(), words[2].into()),
                "OR" => Self::Or(words[0].into(), words[2].into()),
                "XOR" => Self::Xor(words[0].into(), words[2].into()),
                _ => panic!("bad gate {}", words[1]),
            })
        }
    }
}
