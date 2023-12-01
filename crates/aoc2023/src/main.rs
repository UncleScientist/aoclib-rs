use aoclib::{Runner, Selector};

mod aoc2023_01;

use aoc2023_01::*;

fn main() {
    run_2023(Selector::Last);
}

fn run_2023(which: Selector) {
    let mut day01 = Aoc2023_01::new();

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
