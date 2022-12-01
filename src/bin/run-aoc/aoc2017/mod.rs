use crate::{Runner, Selector};

mod aoc2017_01;
mod aoc2017_02;

use aoc2017_01::*;
use aoc2017_02::*;

pub fn run_2017(which: Selector) {
    let mut day01 = Aoc2017_01::new();
    let mut day02 = Aoc2017_02::new();

    let mut days: Vec<&mut dyn Runner> = vec![&mut day01, &mut day02];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            crate::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                crate::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            crate::run_solution(*d);
        }
    }
}
