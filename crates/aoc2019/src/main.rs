use aoclib::{Runner, Selector};

mod intcode;

mod aoc2019_01;
mod aoc2019_02;
mod aoc2019_03;
mod aoc2019_04;
mod aoc2019_05;
mod aoc2019_06;
mod aoc2019_07;
mod aoc2019_08;
mod aoc2019_09;

use aoc2019_01::*;
use aoc2019_02::*;
use aoc2019_03::*;
use aoc2019_04::*;
use aoc2019_05::*;
use aoc2019_06::*;
use aoc2019_07::*;
use aoc2019_08::*;
use aoc2019_09::*;

pub fn main() {
    run_2019(Selector::Last);
}

pub fn run_2019(which: Selector) {
    let mut day01 = Aoc2019_01::new();
    let mut day02 = Aoc2019_02::new();
    let mut day03 = Aoc2019_03::new();
    let mut day04 = Aoc2019_04::new();
    let mut day05 = Aoc2019_05::new();
    let mut day06 = Aoc2019_06::new();
    let mut day07 = Aoc2019_07::new();
    let mut day08 = Aoc2019_08::new();
    let mut day09 = Aoc2019_09::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09,
    ];

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
