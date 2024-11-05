use aoclib::{Runner, Selector};

mod aoc2019_01;

use aoc2019_01::*;

pub fn main() {
    run_2019(Selector::Last);
}

pub fn run_2019(which: Selector) {
    let mut day01 = Aoc2019_01::new();

    let mut days: Vec<&mut dyn Runner> = vec![&mut day01];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            aoclib::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                aoclib::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            aoclib::run_solution(*d);
        }
    }
}
